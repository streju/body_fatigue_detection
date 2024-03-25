use std::sync::Arc;

use super::proto::common::{Result as CommonResult, Status as CommonStatus};
use super::proto::img_processor::data_processor_server::{
    DataProcessor as ProtoDataProcessor, DataProcessorServer,
};
use super::proto::landmarks::{Eyes, ShouldersCoordinates};
use crate::detection::entry_point::DetectionEntryPoint;
use crate::middleware::translation::{from_proto_eyes, from_proto_shoulders_coordinates};
use crate::middleware::types::DataListener;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Clone)]
pub struct GrpcDataListener {
    data_processor: Arc<DetectionEntryPoint>,
    dest_addr: std::net::SocketAddr,
}

#[tonic::async_trait]
impl ProtoDataProcessor for GrpcDataListener {
    async fn handle_shoulders_coordinates(
        &self,
        request: Request<ShouldersCoordinates>,
    ) -> Result<Response<CommonResult>, Status> {
        println!("HandleShouldersCoordinates, request: {:?}", request);

        self.data_processor
            .as_ref()
            .start_shoulders_analysis(from_proto_shoulders_coordinates(request.get_ref()))
            .await;

        Ok(Response::new(CommonResult {
            status: CommonStatus::Succeeded.into(),
        }))
    }

    async fn handle_eyes_coordinates(
        &self,
        request: Request<Eyes>,
    ) -> Result<Response<CommonResult>, Status> {
        println!("HandleEyesCoordinates, request: {:?}", request);

        self.data_processor
            .start_eyes_analysis(from_proto_eyes(request.get_ref()))
            .await;

        Ok(Response::new(CommonResult {
            status: CommonStatus::Succeeded.into(),
        }))
    }
}

impl GrpcDataListener {
    pub fn new(detection_entry: Arc<DetectionEntryPoint>, dest: &str) -> Box<Self> {
        Box::new(GrpcDataListener {
            data_processor: Arc::clone(&detection_entry),
            dest_addr: dest.parse().unwrap(),
        })
    }
}

#[async_trait::async_trait]
impl DataListener for GrpcDataListener {
    async fn start(self: Box<Self>) -> Result<(), Box<dyn std::error::Error>> {
        let dest = self.dest_addr;
        println!(
            "[GrpcDataListener] Starts listening on: {}",
            &self.dest_addr
        );
        Server::builder()
            .add_service(DataProcessorServer::new(self.as_ref().clone()))
            .serve(dest)
            .await?;
        Ok(())
    }
}
