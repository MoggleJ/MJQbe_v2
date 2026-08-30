FROM debian:bookworm-slim AS builder
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libcjson-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /build
COPY . .
RUN make

FROM debian:bookworm-slim
# libcjson1 : JSON parsing.  cec-utils : `cec-client` for HDMI-CEC (Sprint 8) —
# optional at runtime, the daemon falls back to "cec unavailable" if absent.
RUN apt-get update && apt-get install -y --no-install-recommends \
    libcjson1 \
    cec-utils \
    && rm -rf /var/lib/apt/lists/*
RUN mkdir -p /run/mjqbe /etc/mjqbe
COPY --from=builder /build/mjqbe-daemon /usr/local/bin/mjqbe-daemon
COPY ir-map.json /etc/mjqbe/ir-map.json
CMD ["mjqbe-daemon"]
