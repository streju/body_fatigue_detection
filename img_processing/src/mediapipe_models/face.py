import mediapipe as mp
import time
face_mesh = mp.solutions.face_mesh.FaceMesh() # TODO: configure model
mp_drawing = mp.solutions.drawing_utils
mp_drawing_styles = mp.solutions.drawing_styles
face_label = 'face'
        # self.face_mesh = mp_face_mesh.FaceMesh(
        #     static_image_mode=False,
        #     max_num_faces=1,
        #     refine_landmarks=False,
        #     min_detection_confidence=0.5,
        #     min_tracking_confidence=0.5)
def run_face_landmarks(event, captured_frame, img_with_ladmarks, mp_result):
    results = face_mesh.process(captured_frame)
    if results.multi_face_landmarks:
        for face_landmarks in results.multi_face_landmarks:
            mp_drawing.draw_landmarks(
                image=img_with_ladmarks,
                landmark_list=face_landmarks, # results.multi_face_landmarks[0] instead of lopp
                connections=mp.solutions.face_mesh.FACEMESH_TESSELATION,
                landmark_drawing_spec=mp_drawing.DrawingSpec(color=(0, 255, 0), thickness=1, circle_radius=1), #None
                connection_drawing_spec=mp_drawing.DrawingSpec(color=(0, 255, 0), thickness=1) #mp_drawing_styles.get_default_face_mesh_tesselation_style())
            )

    mp_result['face'] = img_with_ladmarks
    event.set()