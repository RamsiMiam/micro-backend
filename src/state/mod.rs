use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::robot_state::RobotState;

pub type RobotsMap = HashMap<String, RobotState>;
pub type SharedRobots = Arc<RwLock<RobotsMap>>;