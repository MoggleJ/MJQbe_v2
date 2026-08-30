FROM node:20-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build

FROM nginx:alpine
RUN apk add --no-cache gettext curl
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf.template /etc/nginx/templates/http.template
COPY nginx.https.conf.template /etc/nginx/templates/https.template
EXPOSE 80 443
ENV SERVER_NAME=_ HTTPS=0 \
    CERT_PATH=/etc/nginx/certs/cert.pem \
    CERT_KEY_PATH=/etc/nginx/certs/key.pem
CMD ["/bin/sh", "-c", "\
  TPL=/etc/nginx/templates/http.template; \
  [ \"$HTTPS\" = \"1\" ] && TPL=/etc/nginx/templates/https.template; \
  envsubst '${SERVER_NAME} ${CERT_PATH} ${CERT_KEY_PATH}' < $TPL > /etc/nginx/conf.d/default.conf && \
  exec nginx -g 'daemon off;'"]
