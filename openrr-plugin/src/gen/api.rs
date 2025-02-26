// This file is @generated by openrr-internal-codegen.
// It is not intended for manual editing.

#![allow(unused_variables)]
#![allow(clippy::useless_conversion, clippy::unit_arg)]

use arci::{BaseVelocity, Error, Isometry2, Isometry3, WaitFuture};
use abi_stable::StableAbi;
use super::*;
/// The plugin trait.
pub trait Plugin: Send + Sync + 'static {
    /// Creates a new instance of [`arci::Gamepad`] with the specified arguments.
    fn new_gamepad(
        &self,
        args: String,
    ) -> Result<Option<Box<dyn arci::Gamepad>>, arci::Error> {
        let _ = args;
        Ok(None)
    }
    /// Creates a new instance of [`arci::JointTrajectoryClient`] with the specified arguments.
    fn new_joint_trajectory_client(
        &self,
        args: String,
    ) -> Result<Option<Box<dyn arci::JointTrajectoryClient>>, arci::Error> {
        let _ = args;
        Ok(None)
    }
    /// Creates a new instance of [`arci::Localization`] with the specified arguments.
    fn new_localization(
        &self,
        args: String,
    ) -> Result<Option<Box<dyn arci::Localization>>, arci::Error> {
        let _ = args;
        Ok(None)
    }
    /// Creates a new instance of [`arci::MoveBase`] with the specified arguments.
    fn new_move_base(
        &self,
        args: String,
    ) -> Result<Option<Box<dyn arci::MoveBase>>, arci::Error> {
        let _ = args;
        Ok(None)
    }
    /// Creates a new instance of [`arci::Navigation`] with the specified arguments.
    fn new_navigation(
        &self,
        args: String,
    ) -> Result<Option<Box<dyn arci::Navigation>>, arci::Error> {
        let _ = args;
        Ok(None)
    }
    /// Creates a new instance of [`arci::Speaker`] with the specified arguments.
    fn new_speaker(
        &self,
        args: String,
    ) -> Result<Option<Box<dyn arci::Speaker>>, arci::Error> {
        let _ = args;
        Ok(None)
    }
    /// Creates a new instance of [`arci::TransformResolver`] with the specified arguments.
    fn new_transform_resolver(
        &self,
        args: String,
    ) -> Result<Option<Box<dyn arci::TransformResolver>>, arci::Error> {
        let _ = args;
        Ok(None)
    }
}
/// FFI-safe equivalent of [`Box<dyn Plugin>`](Plugin).
#[derive(StableAbi)]
#[repr(C)]
pub struct PluginProxy(pub(crate) crate::proxy::PluginTraitObject);
impl PluginProxy {
    /// Creates a new `PluginProxy`.
    pub fn new<T>(inner: T) -> Self
    where
        T: Plugin + 'static,
    {
        Self(
            crate::proxy::PluginTraitObject::from_value(
                inner,
                abi_stable::erased_types::TD_Opaque,
            ),
        )
    }
}
impl PluginProxy {
    /// Creates a new instance of [`arci::Gamepad`] with the specified arguments.
    pub fn new_gamepad(
        &self,
        args: String,
    ) -> Result<Option<GamepadProxy>, arci::Error> {
        Ok(self.0.new_gamepad(args.into()).into_result()?.into_option())
    }
    /// Creates a new instance of [`arci::JointTrajectoryClient`] with the specified arguments.
    pub fn new_joint_trajectory_client(
        &self,
        args: String,
    ) -> Result<Option<JointTrajectoryClientProxy>, arci::Error> {
        Ok(self.0.new_joint_trajectory_client(args.into()).into_result()?.into_option())
    }
    /// Creates a new instance of [`arci::Localization`] with the specified arguments.
    pub fn new_localization(
        &self,
        args: String,
    ) -> Result<Option<LocalizationProxy>, arci::Error> {
        Ok(self.0.new_localization(args.into()).into_result()?.into_option())
    }
    /// Creates a new instance of [`arci::MoveBase`] with the specified arguments.
    pub fn new_move_base(
        &self,
        args: String,
    ) -> Result<Option<MoveBaseProxy>, arci::Error> {
        Ok(self.0.new_move_base(args.into()).into_result()?.into_option())
    }
    /// Creates a new instance of [`arci::Navigation`] with the specified arguments.
    pub fn new_navigation(
        &self,
        args: String,
    ) -> Result<Option<NavigationProxy>, arci::Error> {
        Ok(self.0.new_navigation(args.into()).into_result()?.into_option())
    }
    /// Creates a new instance of [`arci::Speaker`] with the specified arguments.
    pub fn new_speaker(
        &self,
        args: String,
    ) -> Result<Option<SpeakerProxy>, arci::Error> {
        Ok(self.0.new_speaker(args.into()).into_result()?.into_option())
    }
    /// Creates a new instance of [`arci::TransformResolver`] with the specified arguments.
    pub fn new_transform_resolver(
        &self,
        args: String,
    ) -> Result<Option<TransformResolverProxy>, arci::Error> {
        Ok(self.0.new_transform_resolver(args.into()).into_result()?.into_option())
    }
}
/// FFI-safe equivalent of [`Box<dyn arci::Localization>`](arci::Localization).
#[derive(StableAbi)]
#[repr(C)]
pub struct LocalizationProxy(pub(crate) crate::proxy::LocalizationTraitObject);
impl LocalizationProxy {
    /// Creates a new `LocalizationProxy`.
    pub fn new<T>(inner: T) -> Self
    where
        T: arci::Localization + 'static,
    {
        Self(
            crate::proxy::LocalizationTraitObject::from_value(
                inner,
                abi_stable::erased_types::TD_Opaque,
            ),
        )
    }
}
impl arci::Localization for LocalizationProxy {
    fn current_pose(&self, frame_id: &str) -> Result<Isometry2<f64>, Error> {
        Ok(self.0.current_pose(frame_id.into()).into_result()?.into())
    }
}
impl std::fmt::Debug for LocalizationProxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalizationProxy").finish()
    }
}
/// FFI-safe equivalent of [`Box<dyn arci::MoveBase>`](arci::MoveBase).
#[derive(StableAbi)]
#[repr(C)]
pub struct MoveBaseProxy(pub(crate) crate::proxy::MoveBaseTraitObject);
impl MoveBaseProxy {
    /// Creates a new `MoveBaseProxy`.
    pub fn new<T>(inner: T) -> Self
    where
        T: arci::MoveBase + 'static,
    {
        Self(
            crate::proxy::MoveBaseTraitObject::from_value(
                inner,
                abi_stable::erased_types::TD_Opaque,
            ),
        )
    }
}
impl arci::MoveBase for MoveBaseProxy {
    fn send_velocity(&self, velocity: &BaseVelocity) -> Result<(), Error> {
        Ok(self.0.send_velocity((*velocity).into()).into_result()?.into())
    }
    fn current_velocity(&self) -> Result<BaseVelocity, Error> {
        Ok(self.0.current_velocity().into_result()?.into())
    }
}
impl std::fmt::Debug for MoveBaseProxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MoveBaseProxy").finish()
    }
}
/// FFI-safe equivalent of [`Box<dyn arci::Navigation>`](arci::Navigation).
#[derive(StableAbi)]
#[repr(C)]
pub struct NavigationProxy(pub(crate) crate::proxy::NavigationTraitObject);
impl NavigationProxy {
    /// Creates a new `NavigationProxy`.
    pub fn new<T>(inner: T) -> Self
    where
        T: arci::Navigation + 'static,
    {
        Self(
            crate::proxy::NavigationTraitObject::from_value(
                inner,
                abi_stable::erased_types::TD_Opaque,
            ),
        )
    }
}
impl arci::Navigation for NavigationProxy {
    fn send_goal_pose(
        &self,
        goal: Isometry2<f64>,
        frame_id: &str,
        timeout: std::time::Duration,
    ) -> Result<WaitFuture, Error> {
        Ok(
            self
                .0
                .send_goal_pose(goal.into(), frame_id.into(), timeout.into())
                .into_result()?
                .into(),
        )
    }
    fn cancel(&self) -> Result<(), Error> {
        Ok(self.0.cancel().into_result()?.into())
    }
}
impl std::fmt::Debug for NavigationProxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NavigationProxy").finish()
    }
}
/// FFI-safe equivalent of [`Box<dyn arci::Speaker>`](arci::Speaker).
#[derive(StableAbi)]
#[repr(C)]
pub struct SpeakerProxy(pub(crate) crate::proxy::SpeakerTraitObject);
impl SpeakerProxy {
    /// Creates a new `SpeakerProxy`.
    pub fn new<T>(inner: T) -> Self
    where
        T: arci::Speaker + 'static,
    {
        Self(
            crate::proxy::SpeakerTraitObject::from_value(
                inner,
                abi_stable::erased_types::TD_Opaque,
            ),
        )
    }
}
impl arci::Speaker for SpeakerProxy {
    fn speak(&self, message: &str) -> Result<WaitFuture, Error> {
        Ok(self.0.speak(message.into()).into_result()?.into())
    }
}
impl std::fmt::Debug for SpeakerProxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpeakerProxy").finish()
    }
}
/// FFI-safe equivalent of [`Box<dyn arci::TransformResolver>`](arci::TransformResolver).
#[derive(StableAbi)]
#[repr(C)]
pub struct TransformResolverProxy(pub(crate) crate::proxy::TransformResolverTraitObject);
impl TransformResolverProxy {
    /// Creates a new `TransformResolverProxy`.
    pub fn new<T>(inner: T) -> Self
    where
        T: arci::TransformResolver + 'static,
    {
        Self(
            crate::proxy::TransformResolverTraitObject::from_value(
                inner,
                abi_stable::erased_types::TD_Opaque,
            ),
        )
    }
}
impl arci::TransformResolver for TransformResolverProxy {
    fn resolve_transformation(
        &self,
        from: &str,
        to: &str,
        time: std::time::SystemTime,
    ) -> Result<Isometry3<f64>, Error> {
        Ok(
            self
                .0
                .resolve_transformation(from.into(), to.into(), time.try_into()?)
                .into_result()?
                .into(),
        )
    }
}
impl std::fmt::Debug for TransformResolverProxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TransformResolverProxy").finish()
    }
}
