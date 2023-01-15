use arci::*;
use parking_lot::Mutex;
#[cfg(feature = "r2r")]
use r2r::geometry_msgs::msg::Twist as r2r_Twist;
#[cfg(all(feature = "safe_drive", not(feature = "r2r")))]
use safe_drive::msg::common_interfaces::geometry_msgs::msg::Twist as safe_drive_Twist;
use serde::{Deserialize, Serialize};
#[cfg(all(feature = "safe_drive", not(feature = "r2r")))]
use std::sync::Arc;

/// Implement arci::MoveBase for ROS2
#[cfg(feature = "r2r")]
pub struct Ros2CmdVelMoveBase {
    vel_publisher: Mutex<r2r::Publisher<r2r_Twist>>,
    // keep not to be dropped
    _node: Mutex<r2r::Node>,
}

#[cfg(all(feature = "safe_drive", not(feature = "r2r")))]
pub struct Ros2CmdVelMoveBase {
    vel_publisher: Mutex<safe_drive::topic::publisher::Publisher<safe_drive_Twist>>,
    _node: Mutex<safe_drive::node::Node>,
}

#[cfg(feature = "r2r")]
impl Ros2CmdVelMoveBase {
    /// Creates a new `Ros2CmdVelMoveBase` from ROS2 context and Twist topic name.
    #[track_caller]
    pub fn new(ctx: r2r::Context, cmd_topic_name: &str) -> Self {
        // TODO: Consider using unique name
        let node = r2r::Node::create(ctx, "cmd_vel_node", "arci_ros2").unwrap();
        Self::from_node(node, cmd_topic_name)
    }

    /// Creates a new `Ros2CmdVelMoveBase` from ROS2 node and Twist topic name.
    #[track_caller]
    pub fn from_node(mut node: r2r::Node, cmd_topic_name: &str) -> Self {
        Self {
            vel_publisher: Mutex::new(
                node.create_publisher(cmd_topic_name, r2r::QosProfile::default())
                    .unwrap(),
            ),
            _node: Mutex::new(node),
        }
    }
}

#[cfg(all(feature = "safe_drive", not(feature = "r2r")))]
impl Ros2CmdVelMoveBase {
    /// Creates a new `Ros2CmdVelMoveBase` from ROS2 context and Twist topic name.
    #[track_caller]
    pub fn new(ctx: Arc<safe_drive::context::Context>, cmd_topic_name: &str) -> Self {
        // TODO: Consider using unique name
        let node = ctx.create_node("cmd_vel_node", None, Default::default()).unwrap();
        Self::from_node(node, cmd_topic_name)
    }

    /// Creates a new `Ros2CmdVelMoveBase` from ROS2 node and Twist topic name.
    #[track_caller]
    pub fn from_node(mut node: Arc<safe_drive::node::Node>, cmd_topic_name: &str) -> Self {
        Self {
            vel_publisher: Mutex::new(
                node.create_publisher::<safe_drive_Twist>(cmd_topic_name, None)
                    .unwrap(),
            ),
            _node: Mutex::new(*node.as_ref()),
        }
    }
}

#[cfg(feature = "r2r")]
impl MoveBase for Ros2CmdVelMoveBase {
    fn send_velocity(&self, velocity: &BaseVelocity) -> Result<(), Error> {
        let mut twist_msg = r2r_Twist::default();
        twist_msg.linear.x = velocity.x;
        twist_msg.linear.y = velocity.y;
        twist_msg.angular.z = velocity.theta;
        self.vel_publisher
            .lock()
            .publish(&twist_msg)
            .map_err(|e| arci::Error::Connection {
                message: format!("r2r publish error: {e:?}"),
            })
    }

    fn current_velocity(&self) -> Result<BaseVelocity, Error> {
        unimplemented!("Read from /odom in the future?");
    }
}

#[cfg(all(feature = "safe_drive", not(feature = "r2r")))]
impl MoveBase for Ros2CmdVelMoveBase {
    fn send_velocity(&self, velocity: &BaseVelocity) -> Result<(), Error> {
        let mut twist_msg = safe_drive_Twist::new().unwrap();
        twist_msg.linear.x = velocity.x;
        twist_msg.linear.y = velocity.y;
        twist_msg.angular.z = velocity.theta;
        self.vel_publisher
            .lock()
            .send(&twist_msg)
            .map_err(|e| arci::Error::Connection {
                message: format!("safe_drive publish error: {e:?}"),
            })
    }

    fn current_velocity(&self) -> Result<BaseVelocity, Error> {
        unimplemented!("Read from /odom in the future?");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
/// Config for Ros2CmdVelMoveBaseConfig
pub struct Ros2CmdVelMoveBaseConfig {
    /// topic name for Twist
    pub topic: String,
}
