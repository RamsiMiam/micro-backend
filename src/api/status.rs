use axum::Json;

use crate::models::robot_state::RobotState;

pub async fn status() -> Json<RobotState> {
    Json(RobotState {
        id: "30AEA4FF0601".to_string(),
        x: 1.0,
        y: 2.0,
        theta: 0.5,
        motor_velocities: [0.5, 0.5, 0.5, 0.5],
    })
}