import cv2
import queue
import threading
import numpy as np
from common import make_proto_cordinate, raw_img_type, debug_landmarks_img_type
from mediapipe_models.executor import Executor, face_label, pose_label

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
        
            left_shoulder_result = mp_result['pose']['left_shoulder']
            right_shoulder_result = mp_result['pose']['right_shoulder']
            
            left_shoulder_coords = None 
            right_shoulder_coords = None
            if left_shoulder_result:
                left_shoulder_coords = make_proto_cordinate(left_shoulder_result.x, left_shoulder_result.y)
            if right_shoulder_result:
                right_shoulder_coords = make_proto_cordinate(right_shoulder_result.x, right_shoulder_result.y)

            middleware_client.publish_shoulders_coordinates(left_shoulder_coords, right_shoulder_coords)