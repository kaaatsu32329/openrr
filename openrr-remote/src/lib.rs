#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, rust_2018_idioms)]
// This lint is unable to correctly determine if an atomic is sufficient to replace the mutex use.
// https://github.com/rust-lang/rust-clippy/issues/4295
#![allow(clippy::mutex_atomic)]

mod pb {
    include!("generated/arci.rs");
}

#[path = "gen/impls.rs"]
mod impls;

use std::{
    future::Future,
    net::SocketAddr,
    time::{Duration, SystemTime},
};

use arci::nalgebra;
use tracing::error;

pub use crate::impls::*;

fn block_in_place<T>(f: impl Future<Output = T>) -> T {
    tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(f))
}

fn wait_from_handle(
    handle: tokio::task::JoinHandle<Result<tonic::Response<()>, tonic::Status>>,
) -> arci::WaitFuture {
    arci::WaitFuture::new(async move {
        handle
            .await
            .map_err(|e| arci::Error::Other(e.into()))?
            .map_err(|e| arci::Error::Other(e.into()))?;
        Ok(())
    })
}

// =============================================================================
// arci::JointTrajectoryClient

impl arci::JointTrajectoryClient for RemoteJointTrajectoryClientSender {
    fn joint_names(&self) -> Vec<String> {
        let mut client = self.client.clone();
        block_in_place(client.joint_names(()))
            .unwrap()
            .into_inner()
            .names
    }

    fn current_joint_positions(&self) -> Result<Vec<f64>, arci::Error> {
        let mut client = self.client.clone();
        Ok(block_in_place(client.current_joint_positions(()))
            .map_err(|e| arci::Error::Other(e.into()))?
            .into_inner()
            .positions)
    }

    fn send_joint_positions(
        &self,
        positions: Vec<f64>,
        duration: Duration,
    ) -> Result<arci::WaitFuture, arci::Error> {
        let mut client = self.client.clone();
        Ok(wait_from_handle(tokio::spawn(async move {
            client
                .send_joint_positions(pb::JointPositionsRequest {
                    positions,
                    duration: Some(duration.into()),
                })
                .await
        })))
    }

    fn send_joint_trajectory(
        &self,
        trajectory: Vec<arci::TrajectoryPoint>,
    ) -> Result<arci::WaitFuture, arci::Error> {
        let mut client = self.client.clone();
        Ok(wait_from_handle(tokio::spawn(async move {
            client
                .send_joint_trajectory(pb::JointTrajectoryRequest {
                    trajectory: trajectory.into_iter().map(Into::into).collect(),
                })
                .await
        })))
    }
}

#[tonic::async_trait]
impl<C> pb::joint_trajectory_client_server::JointTrajectoryClient
    for RemoteJointTrajectoryClientReceiver<C>
where
    C: arci::JointTrajectoryClient + 'static,
{
    async fn joint_names(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<pb::JointNamesResponse>, tonic::Status> {
        Ok(tonic::Response::new(pb::JointNamesResponse {
            names: arci::JointTrajectoryClient::joint_names(&self.inner),
        }))
    }

    async fn current_joint_positions(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<pb::JointPositionsResponse>, tonic::Status> {
        Ok(tonic::Response::new(pb::JointPositionsResponse {
            positions: arci::JointTrajectoryClient::current_joint_positions(&self.inner)
                .map_err(|e| tonic::Status::unknown(e.to_string()))?,
        }))
    }

    async fn send_joint_positions(
        &self,
        request: tonic::Request<pb::JointPositionsRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();
        arci::JointTrajectoryClient::send_joint_positions(
            &self.inner,
            request.positions,
            request.duration.unwrap().try_into().unwrap(),
        )
        .map_err(|e| tonic::Status::unknown(e.to_string()))?
        .await
        .map_err(|e| tonic::Status::unknown(e.to_string()))?;
        Ok(tonic::Response::new(()))
    }

    async fn send_joint_trajectory(
        &self,
        request: tonic::Request<pb::JointTrajectoryRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();
        arci::JointTrajectoryClient::send_joint_trajectory(
            &self.inner,
            request.trajectory.into_iter().map(Into::into).collect(),
        )
        .map_err(|e| tonic::Status::unknown(e.to_string()))?
        .await
        .map_err(|e| tonic::Status::unknown(e.to_string()))?;
        Ok(tonic::Response::new(()))
    }
}

// =============================================================================
// arci::Gamepad

#[arci::async_trait]
impl arci::Gamepad for RemoteGamepadSender {
    async fn next_event(&self) -> arci::gamepad::GamepadEvent {
        let mut client = self.client.clone();
        match client.next_event(()).await {
            Ok(event) => event.into_inner().into(),
            Err(e) => {
                error!("{}", e);
                arci::gamepad::GamepadEvent::Unknown
            }
        }
    }

    fn stop(&self) {
        let mut client = self.client.clone();
        if let Err(e) = block_in_place(client.stop(())) {
            error!("{}", e);
        }
    }
}

#[tonic::async_trait]
impl<C> pb::gamepad_server::Gamepad for RemoteGamepadReceiver<C>
where
    C: arci::Gamepad + 'static,
{
    async fn next_event(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<pb::GamepadEvent>, tonic::Status> {
        Ok(tonic::Response::new(self.inner.next_event().await.into()))
    }

    async fn stop(&self, _: tonic::Request<()>) -> Result<tonic::Response<()>, tonic::Status> {
        self.inner.stop();
        Ok(tonic::Response::new(()))
    }
}

// =============================================================================
// Messages

impl From<arci::TrajectoryPoint> for pb::TrajectoryPoint {
    fn from(val: arci::TrajectoryPoint) -> Self {
        Self {
            positions: val.positions,
            velocities: val.velocities.unwrap_or_default(),
            time_from_start: Some(val.time_from_start.into()),
        }
    }
}

impl From<pb::TrajectoryPoint> for arci::TrajectoryPoint {
    fn from(val: pb::TrajectoryPoint) -> Self {
        Self {
            positions: val.positions,
            velocities: if val.velocities.is_empty() {
                None
            } else {
                Some(val.velocities)
            },
            time_from_start: val.time_from_start.unwrap().try_into().unwrap(),
        }
    }
}

impl From<arci::BaseVelocity> for pb::BaseVelocity {
    fn from(val: arci::BaseVelocity) -> Self {
        Self {
            x: val.x,
            y: val.y,
            theta: val.theta,
        }
    }
}

impl From<pb::BaseVelocity> for arci::BaseVelocity {
    fn from(val: pb::BaseVelocity) -> Self {
        Self {
            x: val.x,
            y: val.y,
            theta: val.theta,
        }
    }
}

impl From<arci::Isometry2<f64>> for pb::Isometry2 {
    fn from(val: arci::Isometry2<f64>) -> Self {
        Self {
            rotation: Some(pb::UnitComplex {
                re: val.rotation.re,
                im: val.rotation.im,
            }),
            translation: Some(pb::Translation2 {
                x: val.translation.x,
                y: val.translation.y,
            }),
        }
    }
}

impl From<pb::Isometry2> for arci::Isometry2<f64> {
    fn from(val: pb::Isometry2) -> Self {
        let translation = val.translation.unwrap();
        let rotation = val.rotation.unwrap();
        Self::from_parts(
            nalgebra::Translation2::new(translation.x, translation.y),
            nalgebra::UnitComplex::from_complex(nalgebra::Complex {
                re: rotation.re,
                im: rotation.im,
            }),
        )
    }
}

impl From<arci::Isometry3<f64>> for pb::Isometry3 {
    fn from(val: arci::Isometry3<f64>) -> Self {
        Self {
            rotation: Some(pb::UnitQuaternion {
                x: val.rotation.coords.x,
                y: val.rotation.coords.y,
                z: val.rotation.coords.z,
                w: val.rotation.coords.w,
            }),
            translation: Some(pb::Translation3 {
                x: val.translation.x,
                y: val.translation.y,
                z: val.translation.z,
            }),
        }
    }
}

impl From<pb::Isometry3> for arci::Isometry3<f64> {
    fn from(val: pb::Isometry3) -> Self {
        let translation = val.translation.unwrap();
        let rotation = val.rotation.unwrap();
        Self::from_parts(
            nalgebra::Translation3::new(translation.x, translation.y, translation.z),
            nalgebra::UnitQuaternion::from_quaternion(nalgebra::Quaternion::new(
                rotation.w, rotation.x, rotation.y, rotation.z,
            )),
        )
    }
}

impl From<(arci::Isometry2<f64>, &str, Duration)> for pb::GoalPoseRequest {
    fn from((goal, frame_id, timeout): (arci::Isometry2<f64>, &str, Duration)) -> Self {
        Self {
            goal: Some(goal.into()),
            frame_id: frame_id.into(),
            timeout: Some(timeout.into()),
        }
    }
}

impl From<(&str, &str, SystemTime)> for pb::ResolveTransformationRequest {
    fn from((from, to, time): (&str, &str, SystemTime)) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            time: Some(time.into()),
        }
    }
}

impl From<arci::gamepad::GamepadEvent> for pb::GamepadEvent {
    fn from(val: arci::gamepad::GamepadEvent) -> Self {
        let event = match val {
            arci::gamepad::GamepadEvent::ButtonPressed(b) => {
                pb::gamepad_event::Event::ButtonPressed(pb::Button::from(b) as _)
            }
            arci::gamepad::GamepadEvent::ButtonReleased(b) => {
                pb::gamepad_event::Event::ButtonReleased(pb::Button::from(b) as _)
            }
            arci::gamepad::GamepadEvent::AxisChanged(axis, value) => {
                pb::gamepad_event::Event::AxisChanged(pb::AxisChanged {
                    axis: pb::Axis::from(axis) as _,
                    value,
                })
            }
            arci::gamepad::GamepadEvent::Connected => pb::gamepad_event::Event::Connected(()),
            arci::gamepad::GamepadEvent::Disconnected => pb::gamepad_event::Event::Disconnected(()),
            arci::gamepad::GamepadEvent::Unknown => pb::gamepad_event::Event::Unknown(()),
        };
        Self { event: Some(event) }
    }
}

impl From<arci::gamepad::Button> for pb::Button {
    fn from(val: arci::gamepad::Button) -> Self {
        match val {
            arci::gamepad::Button::South => Self::South,
            arci::gamepad::Button::East => Self::East,
            arci::gamepad::Button::North => Self::North,
            arci::gamepad::Button::West => Self::West,
            arci::gamepad::Button::LeftTrigger => Self::LeftTrigger,
            arci::gamepad::Button::LeftTrigger2 => Self::LeftTrigger2,
            arci::gamepad::Button::RightTrigger => Self::RightTrigger,
            arci::gamepad::Button::RightTrigger2 => Self::RightTrigger2,
            arci::gamepad::Button::Select => Self::Select,
            arci::gamepad::Button::Start => Self::Start,
            arci::gamepad::Button::Mode => Self::Mode,
            arci::gamepad::Button::LeftThumb => Self::LeftThumb,
            arci::gamepad::Button::RightThumb => Self::RightThumb,
            arci::gamepad::Button::DPadUp => Self::DPadUp,
            arci::gamepad::Button::DPadDown => Self::DPadDown,
            arci::gamepad::Button::DPadLeft => Self::DPadLeft,
            arci::gamepad::Button::DPadRight => Self::DPadRight,
            arci::gamepad::Button::Unknown => Self::Unknown,
        }
    }
}

impl From<arci::gamepad::Axis> for pb::Axis {
    fn from(val: arci::gamepad::Axis) -> Self {
        match val {
            arci::gamepad::Axis::LeftStickX => Self::LeftStickX,
            arci::gamepad::Axis::LeftStickY => Self::LeftStickY,
            arci::gamepad::Axis::LeftTrigger => Self::LeftTrigger,
            arci::gamepad::Axis::RightStickX => Self::RightStickX,
            arci::gamepad::Axis::RightStickY => Self::RightStickY,
            arci::gamepad::Axis::RightTrigger => Self::RightTrigger,
            arci::gamepad::Axis::DPadX => Self::DPadX,
            arci::gamepad::Axis::DPadY => Self::DPadY,
            arci::gamepad::Axis::Unknown => Self::Unknown,
        }
    }
}

impl From<pb::GamepadEvent> for arci::gamepad::GamepadEvent {
    fn from(val: pb::GamepadEvent) -> Self {
        let val = val.event.unwrap();
        match val {
            pb::gamepad_event::Event::ButtonPressed(b) => {
                Self::ButtonPressed(pb::Button::from_i32(b).unwrap().into())
            }
            pb::gamepad_event::Event::ButtonReleased(b) => {
                Self::ButtonReleased(pb::Button::from_i32(b).unwrap().into())
            }
            pb::gamepad_event::Event::AxisChanged(a) => {
                Self::AxisChanged(pb::Axis::from_i32(a.axis).unwrap().into(), a.value)
            }
            pb::gamepad_event::Event::Connected(()) => Self::Connected,
            pb::gamepad_event::Event::Disconnected(()) => Self::Disconnected,
            pb::gamepad_event::Event::Unknown(()) => Self::Unknown,
        }
    }
}

impl From<pb::Button> for arci::gamepad::Button {
    fn from(val: pb::Button) -> Self {
        match val {
            pb::Button::South => Self::South,
            pb::Button::East => Self::East,
            pb::Button::North => Self::North,
            pb::Button::West => Self::West,
            pb::Button::LeftTrigger => Self::LeftTrigger,
            pb::Button::LeftTrigger2 => Self::LeftTrigger2,
            pb::Button::RightTrigger => Self::RightTrigger,
            pb::Button::RightTrigger2 => Self::RightTrigger2,
            pb::Button::Select => Self::Select,
            pb::Button::Start => Self::Start,
            pb::Button::Mode => Self::Mode,
            pb::Button::LeftThumb => Self::LeftThumb,
            pb::Button::RightThumb => Self::RightThumb,
            pb::Button::DPadUp => Self::DPadUp,
            pb::Button::DPadDown => Self::DPadDown,
            pb::Button::DPadLeft => Self::DPadLeft,
            pb::Button::DPadRight => Self::DPadRight,
            pb::Button::Unknown => Self::Unknown,
        }
    }
}

impl From<pb::Axis> for arci::gamepad::Axis {
    fn from(val: pb::Axis) -> Self {
        match val {
            pb::Axis::LeftStickX => Self::LeftStickX,
            pb::Axis::LeftStickY => Self::LeftStickY,
            pb::Axis::LeftTrigger => Self::LeftTrigger,
            pb::Axis::RightStickX => Self::RightStickX,
            pb::Axis::RightStickY => Self::RightStickY,
            pb::Axis::RightTrigger => Self::RightTrigger,
            pb::Axis::DPadX => Self::DPadX,
            pb::Axis::DPadY => Self::DPadY,
            pb::Axis::Unknown => Self::Unknown,
        }
    }
}
