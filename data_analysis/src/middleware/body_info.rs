use super::translation::to_proto_blinking;
use super::types::VisualizationClient;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct BodyInfo {
    visualization_client: Arc<Mutex<dyn VisualizationClient>>,
}

impl BodyInfo {
    pub fn new(visualization: &Arc<Mutex<dyn VisualizationClient>>) -> Arc<Self> {
        Arc::new(BodyInfo {
            visualization_client: visualization.clone(),
        })
    }
    pub async fn send_blinking_counter_update(self: &Self, counter: u32) {
        self.visualization_client
            .lock()
            .await
            .send_blinking_counter(to_proto_blinking(counter))
            .await
            .unwrap();
    }
}
