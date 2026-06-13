use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::robot_state::RobotState;
use crate::mqtt::publisher::MqttPublisher;

pub type RobotsMap = HashMap<String, RobotState>;
pub type SharedRobots = Arc<RwLock<RobotsMap>>;

#[derive(Clone)]
pub struct AppState {
    pub robots: SharedRobots,
    pub mqtt_publisher: MqttPublisher,
}