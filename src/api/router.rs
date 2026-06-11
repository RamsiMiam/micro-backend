use axum::{
    routing::get,
    Router,
};

use crate::state::SharedRobots;

use super::robots::robots;

pub fn create_router() -> Router<SharedRobots> {
    Router::new()
        .route("/robots", get(robots))
}