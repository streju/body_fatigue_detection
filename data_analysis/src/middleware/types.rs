use async_trait::async_trait;

#[async_trait]
pub trait DataListener {
    async fn start(self: Box<Self>) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait VisualizationClient: Sync + Send {
    async fn send_shoulders_info(self: &Self) -> Result<(), Box<dyn std::error::Error>>;
}
