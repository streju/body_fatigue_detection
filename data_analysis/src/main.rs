mod alerts_reporter;
mod config_parser;
mod detection;
mod middleware;

use crate::detection::entry_point::DetectionEntryPoint;
use crate::middleware::body_info::BodyInfo;
use crate::middleware::builder::Builder as MiddlewareBuilder;
use alerts_reporter::AlertsReporter;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = MiddlewareBuilder::create_from_config_file("config.json");
    let visualization_client = builder.get_visualization_client().await?;
    let alerts_reporter = AlertsReporter::new(&visualization_client);
    let body_info = BodyInfo::new(&visualization_client);
    let detection_entry: Arc<DetectionEntryPoint> =
        DetectionEntryPoint::new(&alerts_reporter, &body_info);
    let data_listener = builder.get_data_listener(detection_entry);
    data_listener.start().await // blocking tokio runtime
}
