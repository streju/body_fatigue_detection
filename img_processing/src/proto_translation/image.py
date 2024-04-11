import numpy as np

import proto_generated.image_pb2 as image_pb2
from proto_translation.common import make_proto_data_header

raw_img_type = 0
debug_landmarks_img_type = 1


def make_proto_image(image: np.ndarray, img_type: int):
    (h, w, ch) = image.shape
    return image_pb2.Image(
        width=w,
        height=h,
        channels=ch,
        data=image.tobytes(),
        type=img_type)


def make_proto_camera_frame(image: np.ndarray, img_type, camera_id=0):
    camera_info = image_pb2.CameraInfo(
        cameraId=str(camera_id)
    )
    return image_pb2.CameraFrame(
        header=make_proto_data_header(),
        cameraInfo=camera_info,
        image=make_proto_image(image, img_type)
    )