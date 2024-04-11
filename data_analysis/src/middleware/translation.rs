use super::grpc::proto::alerts::{
    Action as ProtoAction, Alert as ProtoAlert, AlertType as ProtoAlertType,
};
use super::grpc::proto::landmarks::{
    Coordinates as ProtoCoordinates, Eye as ProtoEye, Eyes as ProtoEyes, Iris as ProtoIris,
    ShouldersCoordinates as ProtoShouldersCoordinates,
};

use super::grpc::proto::body_info::Blinking;

use crate::alerts_reporter::AlertType;
use crate::detection::types::{Coordinates, Eye, EyesInput, Iris, ShouldersCoordinatesInput};

pub fn from_proto_shoulders_coordinates(
    proto_shoulders: &ProtoShouldersCoordinates,
) -> ShouldersCoordinatesInput {
    ShouldersCoordinatesInput {
        left_shoulder: translate_single_coordinate(&proto_shoulders.left_shoulder),
        right_shoulder: translate_single_coordinate(&proto_shoulders.right_shoulder),
    }
}

fn translate_single_coordinate(coord_opt: &Option<ProtoCoordinates>) -> Option<Coordinates> {
    match coord_opt {
        Some(coord) => Some(Coordinates {
            x: coord.x,
            y: coord.y,
        }),
        None => None,
    }
}

pub fn to_proto_alert(alert_type: &AlertType, action: bool) -> ProtoAlert {
    let proto_alert_action: ProtoAction = if action {
        ProtoAction::Start
    } else {
        ProtoAction::Stop
    };
    match alert_type {
        AlertType::ShoulderPose => ProtoAlert {
            r#type: ProtoAlertType::ShoulderPose.into(),
            action: proto_alert_action.into(),
        },
    }
}

pub fn from_proto_eyes(eyes: &ProtoEyes) -> EyesInput {
    EyesInput {
        right_eye: translate_single_eye(&eyes.right_eye),
        left_eye: translate_single_eye(&eyes.left_eye),
    }
}

fn translate_single_eye(eye_opt: &Option<ProtoEye>) -> Option<Eye> {
    match eye_opt {
        Some(eye) => Some(Eye {
            upper_eyelid: translate_single_coordinate(&eye.upper_eyelid),
            lower_eyelid: translate_single_coordinate(&eye.lower_eyelid),
            iris: translate_iris(&eye.iris),
        }),
        None => None,
    }
}

fn translate_iris(iris_opt: &Option<ProtoIris>) -> Option<Iris> {
    match iris_opt {
        Some(iris) => Some(Iris {
            top: translate_single_coordinate(&iris.top),
            bottom: translate_single_coordinate(&iris.bottom),
            external: translate_single_coordinate(&iris.external),
            interior: translate_single_coordinate(&iris.interior),
        }),
        None => None,
    }
}

pub fn to_proto_blinking(blinking_counter: u32) -> Blinking {
    Blinking {
        counter: blinking_counter as u64,
    }
}
