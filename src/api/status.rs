use axum::{
    extract::State,
    Json,
};

use crate::{
    models::robot_state::RobotState,
    state::SharedRobotState,
};

pub async fn status(
    State(state): State<SharedRobotState>,
) -> Json<RobotState> {

    let robot = state.read().await;

    Json(robot.clone())
}