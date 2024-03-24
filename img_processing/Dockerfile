FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive
RUN adduser --quiet --disabled-password qtuser && usermod -a -G audio -G video qtuser

RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    python3-venv \
    python3-pyqt5 \
    ffmpeg \
    libsm6 \
    libxext6 \
    libgtk2.0-dev \
    pkg-config \
    libnvidia-gl-470-server \
    && apt-get -y autoremove \
    && apt-get clean autoclean \
    && rm -rf /var/lib/apt/lists/*

COPY img_processing /img_processing
# TODO workaround for proper import paths
COPY proto /img_processing/proto/src/proto_generated

# TODO remove before release
COPY proto /proto

WORKDIR /img_processing

ENV VIRTUAL_ENV=/venv
RUN python3 -m venv $VIRTUAL_ENV
ENV PATH="$VIRTUAL_ENV/bin:$PATH"

RUN pip install -r requirements.txt
RUN pip install mediapipe
RUN python fix_proto_import_path.py
RUN ./generate_proto_files.sh

CMD ["python", "src/app.py", "--communication=grpc"]