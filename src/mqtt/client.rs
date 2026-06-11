use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};

use std::time::Duration;
use std::env;

use crate::{models::robot_state::RobotState, state::SharedRobots};

pub async fn mqtt_task(state: SharedRobots) {
    let host = env::var("MQTT_HOST").unwrap_or("localhost".to_string());
    let port: u16 = env::var("MQTT_PORT")
        .unwrap_or("1883".to_string())
        .parse()
        .unwrap();

    let user = env::var("MQTT_USER").unwrap();
    let pass = env::var("MQTT_PASS").unwrap();

    let client_id = env::var("MQTT_CLIENT_ID").unwrap_or("micro_backend".to_string());

    let mut mqttoptions = MqttOptions::new(client_id, host, port);
    mqttoptions.set_credentials(user, pass);

    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client.subscribe("robot/#", QoS::AtMostOnce).await.unwrap();

    println!("MQTT connected");

    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                let payload = String::from_utf8_lossy(&p.payload);

                println!("[MQTT] {} -> {}", p.topic, payload);

                // robot/<mac>/telemetry/pose
                if p.topic.starts_with("robot/") && p.topic.ends_with("/telemetry/pose") {
                    let parts: Vec<&str> = p.topic.split('/').collect();
                    if parts.len() < 4 {
                        println!("Invalid topic format");
                        return;
                    }

                    let mac = parts[1];
                    match serde_json::from_slice::<RobotState>(&p.payload) {
                        Ok(robot_state) => {
                            let mut map = state.write().await;

                            map.entry(mac.to_string())
                                .and_modify(|r| {
                                    *r = robot_state.clone();
                                })
                                .or_insert(robot_state);
                        }

                        Err(e) => {
                            println!("JSON parse error: {:?}", e);
                        }
                    }
                }
            }

            Ok(_) => {}

            Err(e) => {
                println!("MQTT error: {:?}", e);
            }
        }
    }
}
