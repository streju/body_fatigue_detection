import mediapipe as mp
import time

from proto_translation.landmarks_common import Coordinates

face_mesh = mp.solutions.face_mesh.FaceMesh(
    static_image_mode=False,
    max_num_faces=1,
    refine_landmarks=True,
    min_detection_confidence=0.5,
    min_tracking_confidence=0.5)
mp_drawing = mp.solutions.drawing_utils
mp_drawing_styles = mp.solutions.drawing_styles
face_label = 'face'


def create_coords(landmark, img_height, img_width):
    return Coordinates(int(landmark.x * img_width), int(landmark.y * img_height))


def run_face_landmarks(event, captured_frame, img_with_ladmarks, mp_result):
    results = face_mesh.process(captured_frame)

    result = {'eyes': {
        'rightEye': {
            'upperEyelid': None,
            'lowerEyelid': None,
            'iris': {
                'top': None,
                'bottom': None,
                'external': None,
                'interior': None
            }
        },
        'leftEye': {
            'upperEyelid': None,
            'lowerEyelid': None,
            'iris': {
                'top': None,
                'bottom': None,
                'external': None,
                'interior': None
            }
        }
    }}

    if results.multi_face_landmarks:
        face_landmarks = results.multi_face_landmarks[0]
        mp_drawing.draw_landmarks(
            image=img_with_ladmarks,
            landmark_list=face_landmarks,
            connections=mp.solutions.face_mesh.FACEMESH_TESSELATION,
            landmark_drawing_spec=mp_drawing.DrawingSpec(color=(0, 255, 0), thickness=1, circle_radius=1), #None
            connection_drawing_spec=mp_drawing.DrawingSpec(color=(0, 255, 0), thickness=1) #mp_drawing_styles.get_default_face_mesh_tesselation_style())
        )
        mp_drawing.draw_landmarks(
            image=img_with_ladmarks,
            landmark_list=face_landmarks,
            connections=mp.solutions.face_mesh.FACEMESH_CONTOURS,
            landmark_drawing_spec=None,
            connection_drawing_spec=mp_drawing_styles.get_default_face_mesh_contours_style())
        for idx, landmark in enumerate(face_landmarks.landmark):
            img_height = captured_frame.shape[0]
            img_width = captured_frame.shape[1]
            if idx == 159:
                result['eyes']['rightEye']['upperEyelid'] = create_coords(landmark, img_height, img_width)
            elif idx == 145:
                result['eyes']['rightEye']['lowerEyelid'] = create_coords(landmark, img_height, img_width)
            elif idx == 386:
                result['eyes']['leftEye']['upperEyelid'] = create_coords(landmark, img_height, img_width)
            elif idx == 374:
                result['eyes']['leftEye']['lowerEyelid'] = create_coords(landmark, img_height, img_width)
            elif idx == 469:
                result['eyes']['rightEye']['iris']['interior'] = create_coords(landmark, img_height, img_width)
            elif idx == 470:
                result['eyes']['rightEye']['iris']['top'] = create_coords(landmark, img_height, img_width)
            elif idx == 471:
                result['eyes']['rightEye']['iris']['external'] = create_coords(landmark, img_height, img_width)
            elif idx == 472:
                result['eyes']['rightEye']['iris']['bottom'] = create_coords(landmark, img_height, img_width)
            elif idx == 474:
                result['eyes']['leftEye']['iris']['external'] = create_coords(landmark, img_height, img_width)
            elif idx == 475:
                result['eyes']['leftEye']['iris']['top'] = create_coords(landmark, img_height, img_width)
            elif idx == 476:
                result['eyes']['leftEye']['iris']['interior'] = create_coords(landmark, img_height, img_width)
            elif idx == 477:
                result['eyes']['leftEye']['iris']['bottom'] = create_coords(landmark, img_height, img_width)

    mp_result['face'] = result
    event.set()