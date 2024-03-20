use std::sync::Arc;
use tokio::sync::Mutex;

use super::{
    grpc::{listener::GrpcDataListener, visualization_client::Visualization},
    types::{DataListener, VisualizationClient},
};
use crate::config_parser;
use crate::detection::entry_point::DetectionEntryPoint;

pub struct Builder {
    visualization_addr: String,
    data_listener: String,
}

impl Builder {
    pub fn create_from_config_file(config_file_path: &str) -> Self {
        let parser = config_parser::Parser::new(config_file_path).unwrap();

        let img_data_addr: String = String::from("[::1]:4555"); // TODO from config
        let visualization_addr: String = String::from("http://")
            + &parser.config.visualization_server_addr
            + &String::from(":")
            + &parser.config.visualization_server_port.to_string();

        Builder {
            visualization_addr: visualization_addr,
            data_listener: img_data_addr,
        }
    }

    pub fn get_data_listener(
        self: &Self,
        detection_entry: Arc<DetectionEntryPoint>,
    ) -> Box<dyn DataListener> {
        // if grpc
        GrpcDataListener::new(detection_entry, &self.data_listener)
    }

    pub async fn get_visualization_client(
        self: &Self,
    ) -> Result<Arc<Mutex<dyn VisualizationClient>>, tonic::transport::Error> {
        // if grpc
        let vis_client = Visualization::new(self.visualization_addr.clone()).await?;
        Ok(vis_client)
    }
}
