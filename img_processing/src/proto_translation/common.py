import time

import proto_generated.common_pb2 as common_pb2

def make_proto_data_header():
    return common_pb2.MsgHeader(
        timestamp=time.time_ns()
    )