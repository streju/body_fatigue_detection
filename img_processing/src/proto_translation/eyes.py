from typing import Optional

import proto_generated.detected_landmarks_pb2 as detected_landmarks_pb2


def make_proto_iris(top_in: Optional[detected_landmarks_pb2.Coordinates],
                    bottom_in: Optional[detected_landmarks_pb2.Coordinates],
                    external_in: Optional[detected_landmarks_pb2.Coordinates],
                    interior_in: Optional[detected_landmarks_pb2.Coordinates]):
    return detected_landmarks_pb2.Iris(
        top=top_in,
        bottom=bottom_in,
        external=external_in,
        interior=interior_in
    )


def make_proto_eye(upper_eyelid: Optional[detected_landmarks_pb2.Coordinates],
                   lower_eyelid: Optional[detected_landmarks_pb2.Coordinates],
                   iris: Optional[detected_landmarks_pb2.Iris]):
    return detected_landmarks_pb2.Eye(
        upperEyelid=upper_eyelid,
        lowerEyelid=lower_eyelid,
        iris=iris
    )


def make_proto_eyes(right_eye: detected_landmarks_pb2.Eye,
                    left_eye: detected_landmarks_pb2.Eye):
    return detected_landmarks_pb2.Eyes(
        rightEye=right_eye,
        leftEye=left_eye
    )