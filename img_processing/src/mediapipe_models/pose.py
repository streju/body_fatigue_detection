import mediapipe as mp
import cv2
from collections import namedtuple

pose = mp.solutions.pose.Pose() # TODO: configure model
ShoulderCoordinates = namedtuple('ShoulderCoordinates', ['x', 'y'])
pose_label = 'pose'

def run_pose_landmarks(event, captured_frame, img_with_ladmarks, mp_result):
    results = pose.process(captured_frame)

    left_shoulder_coordinates = None
    right_shoulder_coordinates = None
    if results.pose_landmarks:
        for idx, landmark in enumerate(results.pose_landmarks.landmark):
            if idx == 11:
                left_shoulder_coordinates = ShoulderCoordinates(int(landmark.x * captured_frame.shape[1]), int(landmark.y * captured_frame.shape[0]))
            elif idx == 12:
                right_shoulder_coordinates = ShoulderCoordinates(int(landmark.x * captured_frame.shape[1]), int(landmark.y * captured_frame.shape[0]))
    
    if left_shoulder_coordinates and right_shoulder_coordinates:
        cv2.line(img_with_ladmarks, left_shoulder_coordinates, right_shoulder_coordinates, (0, 255, 0), thickness=2)

    mp_result['pose'] = {'left_shoulder': left_shoulder_coordinates, 'right_shoulder': right_shoulder_coordinates}
    event.set()