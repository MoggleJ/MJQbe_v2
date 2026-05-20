FROM node:20-alpine AS builder

WORKDIR /app

COPY package*.json ./
RUN npm install

COPY . .
RUN npm run build

FROM nginx:alpine

RUN apk add --no-cache gettext

COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf.template /etc/nginx/templates/nginx.conf.template

EXPOSE 80

# SERVER_NAME est injecté par docker-compose depuis config.yml
# envsubst ne substitue que ${SERVER_NAME}, les variables nginx ($host etc.) sont préservées
ENV SERVER_NAME=_
CMD ["/bin/sh", "-c", "envsubst '${SERVER_NAME}' < /etc/nginx/templates/nginx.conf.template > /etc/nginx/conf.d/default.conf && nginx -g 'daemon off;'"]
