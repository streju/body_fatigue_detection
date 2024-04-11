use super::proto::alerts::Alert;
use super::proto::body_info::Blinking;
use super::proto::common::Result as ProtoResult;
use super::proto::visualization::visualization_service_client::VisualizationServiceClient;

use crate::middleware::types::VisualizationClient;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Visualization {
    client: VisualizationServiceClient<tonic::transport::Channel>,
}

impl Visualization {
    async fn connect(
        dest: String,
    ) -> Result<VisualizationServiceClient<tonic::transport::Channel>, tonic::transport::Error>
    {
        let client: VisualizationServiceClient<tonic::transport::Channel> =
            VisualizationServiceClient::connect(dest.clone()).await?;
        println!("[VisualizationClient] Connected to server {}!", dest);
        Ok(client)
    }

    pub async fn new(server_addr: String) -> Result<Arc<Mutex<Self>>, tonic::transport::Error> {
        println!(
            "[Visualization] Trying to connect to the server: {}",
            server_addr
        );
        match Self::connect(server_addr).await {
            Ok(vis_client) => Ok(Arc::new(Mutex::new(Visualization { client: vis_client }))),
            Err(err) => Err(err),
        }
    }
}

#[async_trait::async_trait]
impl VisualizationClient for Visualization {
    async fn send_alarm_info(
        self: &mut Self,
        alert: Alert,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(alert);
        let _response: tonic::Response<ProtoResult> =
            self.client.handle_alert_notif(request).await?;
        Ok(())
    }

    async fn send_blinking_counter(
        self: &mut Self,
        blinking: Blinking,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(blinking);
        let _response: tonic::Response<ProtoResult> =
            self.client.handle_blinking_notif(request).await?;
        Ok(())
    }
}
