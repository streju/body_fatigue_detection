import logging
import argparse
import sys
import queue

from image_gatherer import ImageGatherer
from image_consumer import ImageConsumer
from middleware import GrpcClient

def main():
    parser = argparse.ArgumentParser(
        prog="bfd_img_processing",
        description="Captures camera frames and executes mediapipe models on them. Data from processed frame are sent to further processing.",
    )
    
    parser.add_argument(
        "--camera-id",
        type=int,
        default=0,
        help="Camera index"
    )
    
    parser.add_argument(
        "--communication",
        type=str,
        choices=["grpc", "rabbitmq"],
        help="Type of middleware communication",
    )
    
    parser.add_argument(
        "--grpc-server-addr",
        type=str,
        default="[::1]",
        help="Address of gRPC server to communicate"
    )
    
    parser.add_argument(
        "--grpc-server-port",
        type=str,
        default="4555",
        help="Port on which gRPC server listening"
    )
    
    options = parser.parse_args()
    
    if options.communication == "grpc":
        middleware_client = GrpcClient("{}:{}".format(options.grpc_server_addr, options.grpc_server_port))
    else:
        sys.exit('body_fatigue_detection: Not defined type of middleware communication!')

    input_frames = queue.Queue()
    img_consumer=ImageConsumer(input_frames, middleware_client)
    ImageGatherer(input_frames, options.camera_id)    
    
    
if __name__ == "__main__":
    logging.basicConfig()
    main()