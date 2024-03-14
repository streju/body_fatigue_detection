use super::proto::visualization::visualization_service_client::VisualizationServiceClient;
use crate::middleware::types::VisualizationClient;
use std::sync::Arc;

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

    pub async fn new(server_addr: String) -> Result<Arc<Self>, tonic::transport::Error> {
        println!(
            "[Visualization] Trying to connect to the server: {}",
            server_addr
        );
        match Self::connect(server_addr).await {
            Ok(vis_client) => Ok(Arc::new(Visualization { client: vis_client })),
            Err(err) => Err(err),
        }
    }
}

#[async_trait::async_trait]
impl VisualizationClient for Visualization {
    async fn send_shoulders_info(self: &Self) -> Result<(), Box<dyn std::error::Error>> {
        // let request = tonic::Request::new(ShouldersInfo { });
        // let response: tonic::Response<BasicResult> = self.client.send_shoulders_info(request).await?;
        Ok(())
    }
}

unsafe impl Sync for Visualization {}
unsafe impl Send for Visualization {}
