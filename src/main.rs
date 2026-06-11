mod api;
mod models;
mod state;

use std::sync::Arc;

use api::router::create_router;
use models::robot_state::RobotState;
use state::SharedRobotState;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let robot_state: SharedRobotState = Arc::new(RwLock::new(RobotState::new(
        "30AEA4FF0601".to_string(),
        1.0,
        2.0,
        0.5,
        [1.5; 4],
    )));
    let app = create_router()
        .with_state(robot_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
