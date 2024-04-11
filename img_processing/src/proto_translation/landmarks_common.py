from typing import Optional
from collections import namedtuple

import proto_generated.detected_landmarks_pb2 as detected_landmarks_pb2

Coordinates = namedtuple('Coordinates', ['x', 'y'])


def make_proto_cordinate(coords: Optional[Coordinates]):
    return detected_landmarks_pb2.Coordinates(
        x=coords.x,
        y=coords.y
    ) if coords else None