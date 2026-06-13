use axum::{
    Router,
    routing::{get, post},
};
use crate::state::AppState;

use super::robots::{robots, start_robot, stop_robot};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/robots", get(robots))
        .route("/robots/{robot_id}/start", post(start_robot))
        .route("/robots/{robot_id}/stop", post(stop_robot))
}
