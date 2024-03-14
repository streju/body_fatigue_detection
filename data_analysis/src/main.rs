mod config_parser;
mod detection;
mod middleware;

use crate::detection::entry_point::DetectionEntryPoint;
use crate::middleware::builder::Builder as MiddlewareBuilder;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workers_nr: usize = 4;

    let builder = MiddlewareBuilder::create_from_config_file("config.json");
    let visualization_client = builder.get_visualization_client().await?;
    let detection_entry: Arc<DetectionEntryPoint> =
        DetectionEntryPoint::new(workers_nr, &visualization_client);
    let data_listener = builder.get_data_listener(detection_entry);
    data_listener.start().await?;

    Ok(())
}
