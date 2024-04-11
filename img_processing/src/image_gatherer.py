import queue
import cv2

class ImageGatherer():
    def __init__(self, input_frames: queue.Queue, camera_id: int):
        self.run(input_frames, camera_id)
    
    def run(self, input_frames: queue.Queue, camera_id):
        cap = cv2.VideoCapture(camera_id)
        
        while cap.isOpened():
            success, image = cap.read()
            if not success:
                print("Ignoring empty camera frame.")
                continue
            
            image.flags.writeable = False
            image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
            input_frames.put(image)
        
        cap.release()
        cv2.destroyAllWindows()
