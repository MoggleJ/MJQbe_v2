FROM debian:bookworm-slim AS builder
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libcjson-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /build
COPY . .
RUN make

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    libcjson1 \
    && rm -rf /var/lib/apt/lists/*
RUN mkdir -p /run/mjqbe
COPY --from=builder /build/mjqbe-daemon /usr/local/bin/mjqbe-daemon
CMD ["mjqbe-daemon"]
