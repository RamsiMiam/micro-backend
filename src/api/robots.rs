use axum::{
    extract::State,
    Json,
};

use crate::{
    state::RobotsMap,
    state::SharedRobots,
};

pub async fn robots(
    State(state): State<SharedRobots>,
) -> Json<RobotsMap> {

    let map = state.read().await;

    Json(map.clone())
}