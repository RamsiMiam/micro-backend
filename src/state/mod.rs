use std::sync::Arc;

use tokio::sync::RwLock;

use crate::models::robot_state::RobotState;

pub type SharedRobotState = Arc<RwLock<RobotState>>;