import grpc
import numpy as np
from typing import Optional

import proto_generated.image_data_handler_pb2_grpc as image_data_handler_pb2_grpc
import proto_generated.detected_landmarks_pb2 as detected_landmarks_pb2
import proto_generated.visualization_service_pb2_grpc as visualization_service_pb2_grpc
from proto_translation.image import make_proto_camera_frame

class MiddlewareClient():
    def publish_msg(self, frame):
        pass
 
    def publish_shoulders_coordinates(self, left_shoulder_coord, right_shoulder_coord):
        pass


class GrpcClient(MiddlewareClient):
    def __init__(self, client_address):
        self._DataProcessorChannel = grpc.insecure_channel('[::1]:4555') # TODO from config file
        self._DataProcessorStub = image_data_handler_pb2_grpc.DataProcessorStub(self._DataProcessorChannel)
        self._VisualizationChannel = grpc.insecure_channel('localhost:4557') # TODO from config file
        self._VisualizationStub = visualization_service_pb2_grpc.VisualizationServiceStub(self._VisualizationChannel)
        
    def publish_shoulders_coordinates(self, left_shoulder_coord: Optional[detected_landmarks_pb2.Coordinates],
                                      right_shoulder_coord: Optional[detected_landmarks_pb2.Coordinates]):
        shoulders_coords = detected_landmarks_pb2.ShouldersCoordinates(
            leftShoulder=left_shoulder_coord,
            rightShoulder=right_shoulder_coord
        )
        self._DataProcessorStub.HandleShouldersCoordinates(shoulders_coords)
        
    def publish_eyes_coordinates(self, eyes: detected_landmarks_pb2.Eyes):
        self._DataProcessorStub.HandleEyesCoordinates(eyes)

    def publish_camera_frame(self, frame: np.array, img_type: int):
        camera_frame = make_proto_camera_frame(frame, img_type)
        self._VisualizationStub.HandleImageNotif(camera_frame)