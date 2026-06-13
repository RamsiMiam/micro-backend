use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::state::{AppState, RobotsMap};

pub async fn robots(State(state): State<AppState>) -> Json<RobotsMap> {
    let map = state.robots.read().await;

    Json(map.clone())
}

pub async fn start_robot(State(state): State<AppState>, Path(robot_id): Path<String>) -> Result<StatusCode, StatusCode> {
    state
        .mqtt_publisher
        .start_robot(&robot_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::OK)
} 

pub async fn stop_robot(State(state): State<AppState>, Path(robot_id): Path<String>) -> Result<StatusCode, StatusCode> {
    state
        .mqtt_publisher
        .stop_robot(&robot_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::OK)
}
