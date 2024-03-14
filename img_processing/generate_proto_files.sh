#!/bin/bash
source /venv/bin/activate
python -m grpc_tools.protoc \
    -I proto/src/ \
    --python_out=src \
    --pyi_out=src \
    --grpc_python_out=src \
    proto/src/proto_generated/*.proto