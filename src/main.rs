mod api;
mod models;
mod mqtt;
mod state;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use dotenvy::dotenv;
use std::env;

use api::router::create_router;
use state::SharedRobots;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let robots: SharedRobots = Arc::new(RwLock::new(HashMap::new()));

    let mqtt_state = robots.clone();
    let app = create_router().with_state(robots);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tokio::spawn(async move {
        mqtt::client::mqtt_task(mqtt_state).await;
    });

    axum::serve(listener, app).await.unwrap();
}
