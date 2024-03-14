import threading
from typing import List
import numpy as np
from mediapipe_models.face import run_face_landmarks, face_label
from mediapipe_models.pose import run_pose_landmarks, pose_label

class Executor():
    def execute(self, img: np.array, output_img: np.array, label_of_models: List[str]):
        events = []
        threads = []
        mp_result = {}
        for model_label in label_of_models:
            event = threading.Event()
            if model_label == face_label:
                threads.append(threading.Thread(target=run_face_landmarks, args=(event, img, output_img, mp_result)))
            else:
                threads.append(threading.Thread(target=run_pose_landmarks, args=(event, img, output_img, mp_result)))
            events.append(event)
            
        for thread in threads:
            thread.start()
        for event in events:
            event.wait()
        
        return mp_result