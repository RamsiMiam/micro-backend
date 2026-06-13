use std::sync::Arc;

use rumqttc::{AsyncClient, QoS};

#[derive(Clone)]
pub struct MqttPublisher {
    client: Arc<AsyncClient>,
}

impl MqttPublisher {
    pub fn new(client: Arc<AsyncClient>) -> Self {
        Self { client }
    }

    pub async fn start_robot(&self, robot_id: &str) -> Result<(), rumqttc::ClientError> {
        let topic = format!("robot/cmd/{}/start", robot_id);

        self.client
            .publish(topic, QoS::AtLeastOnce, false, "start")
            .await
    }

    pub async fn stop_robot(&self, robot_id: &str) -> Result<(), rumqttc::ClientError> {
        let topic = format!("robot/cmd/{}/stop", robot_id);

        self.client
            .publish(topic, QoS::AtLeastOnce, false, "stop")
            .await
    }

}
