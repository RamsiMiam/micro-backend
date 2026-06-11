use axum::{
    routing::get,
    Router,
};

use crate::state::SharedRobotState;

use super::status::status;

pub fn create_router() -> Router<SharedRobotState> {
    Router::new()
        .route("/status", get(status))
}