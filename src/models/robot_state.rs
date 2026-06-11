use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotState {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    pub motor_velocities: [f32; 4],
}