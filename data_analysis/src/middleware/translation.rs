use super::grpc::proto::landmarks::{Coordinates, ShouldersCoordinates};
use crate::detection::types::{ShoulderCoords, ShouldersCoordinatesInput};

pub fn from_proto_shoulders_coordinates(
    proto_shoulders: &ShouldersCoordinates,
) -> ShouldersCoordinatesInput {
    ShouldersCoordinatesInput {
        left_shoulder: translate_single_shoulder(&proto_shoulders.left_shoulder),
        right_shoulder: translate_single_shoulder(&proto_shoulders.right_shoulder),
    }
}

fn translate_single_shoulder(shoulder_opt: &Option<Coordinates>) -> ShoulderCoords {
    match shoulder_opt {
        Some(shoulder) => ShoulderCoords {
            x: Some(shoulder.x),
            y: Some(shoulder.y),
        },
        None => ShoulderCoords { x: None, y: None },
    }
}
