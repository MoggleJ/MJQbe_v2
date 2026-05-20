FROM python:3.11-slim

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

ARG API_PORT=4848
EXPOSE ${API_PORT}
ENV API_PORT=${API_PORT}
CMD uvicorn app.main:app --host 0.0.0.0 --port ${API_PORT:-4848}
