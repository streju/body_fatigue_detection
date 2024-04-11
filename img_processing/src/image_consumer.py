import queue
import threading
import numpy as np

from mediapipe_models.executor import Executor, face_label, pose_label
from proto_translation.eyes import make_proto_iris, make_proto_eye, make_proto_eyes
from proto_translation.image import raw_img_type, debug_landmarks_img_type
from proto_translation.landmarks_common import make_proto_cordinate

class ImageConsumer():
    def __init__(self, input_frames: queue.Queue, middleware_client):
        self._models_executor = Executor()
        self._working_thread = threading.Thread(target=self._do_work, args=(input_frames, middleware_client))
        self._working_thread.start()

    def _do_work(self, input_frames, middleware_client):
        while True:
            captured_frame = input_frames.get()
            
            middleware_client.publish_camera_frame(captured_frame, raw_img_type)
            img_with_landmarks = np.copy(captured_frame)
            
            mp_result = self._models_executor.execute(captured_frame, img_with_landmarks, [face_label, pose_label])
            middleware_client.publish_camera_frame(img_with_landmarks, debug_landmarks_img_type)
        
            self._distribute_shoulder_coords(mp_result['pose'], middleware_client)
            self._distribute_eyes_coords(mp_result['face']['eyes'], middleware_client)


    def _distribute_shoulder_coords(self, pose_result, middleware_client):
            left_shoulder_result = pose_result['left_shoulder']
            right_shoulder_result = pose_result['right_shoulder']
            
            middleware_client.publish_shoulders_coordinates(
                make_proto_cordinate(left_shoulder_result),
                make_proto_cordinate(right_shoulder_result))

    def _distribute_eyes_coords(self, eyes_result, middleware_client):
        right_eye_result = eyes_result['rightEye']
        left_eye_result = eyes_result['leftEye']

        right_eye_proto = make_proto_eye(make_proto_cordinate(right_eye_result['upperEyelid']),
                                   make_proto_cordinate(right_eye_result['lowerEyelid']),
                                   self._make_proto_iris(right_eye_result['iris']))
        left_eye_proto = make_proto_eye(make_proto_cordinate(left_eye_result['upperEyelid']),
                                   make_proto_cordinate(left_eye_result['lowerEyelid']),
                                   self._make_proto_iris(left_eye_result['iris']))

        middleware_client.publish_eyes_coordinates(make_proto_eyes(right_eye_proto, left_eye_proto))

    def _make_proto_iris(self, iris_result):
        return make_proto_iris(make_proto_cordinate(iris_result['top']),
                               make_proto_cordinate(iris_result['bottom']),
                               make_proto_cordinate(iris_result['external']),
                               make_proto_cordinate(iris_result['interior']))