FROM grpc_cpp:latest

RUN adduser --quiet --disabled-password qtuser && usermod -a -G audio -G video qtuser

RUN apt-get update && apt-get install -y --no-install-recommends \
  libglx-dev \
  libgl1-mesa-dev \
  qt6-base-dev \
  libopencv-dev \
  && apt-get -y autoremove \
  && apt-get clean autoclean \
  && rm -rf /var/lib/apt/lists/*

COPY visualization /visualization
COPY proto /proto
COPY config.json /visualization

WORKDIR /visualization
RUN mkdir -p deps
RUN cd deps && ln -s ../../grpc grpc

RUN cmake -S . -B build
RUN cmake --build build --target all