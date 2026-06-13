mod api;
mod models;
mod mqtt;
mod state;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use dotenvy::dotenv;

use api::router::create_router;
use state::{AppState, SharedRobots};
use mqtt::client::{create_mqtt_client, mqtt_task};

use crate::mqtt::publisher::MqttPublisher;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let robots: SharedRobots = Arc::new(RwLock::new(HashMap::new()));
    let (mqtt_client, eventloop) = create_mqtt_client();
    let mqtt_client = Arc::new(mqtt_client);

    let mqtt_publisher = MqttPublisher::new(mqtt_client.clone());

    let app_state = AppState {
        robots: robots.clone(),
        mqtt_publisher,
    };

    let app = create_router().with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tokio::spawn(async move {
        mqtt_task((*mqtt_client).clone(), eventloop, robots).await;
    });

    axum::serve(listener, app).await.unwrap();
}
