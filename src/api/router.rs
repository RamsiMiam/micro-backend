use axum::{
    routing::get,
    Router,
};

use super::status::status;

pub fn create_router() -> Router {
    Router::new()
        .route("/status", get(status))
}