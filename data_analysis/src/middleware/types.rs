use super::grpc::proto::alerts::Alert;
use super::grpc::proto::body_info::Blinking;
use async_trait::async_trait;

#[async_trait]
pub trait DataListener {
    async fn start(self: Box<Self>) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait VisualizationClient: Sync + Send {
    async fn send_alarm_info(
        self: &mut Self,
        alert: Alert,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn send_blinking_counter(
        self: &mut Self,
        blinking_counter: Blinking,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
