FROM rust:alpine3.22 AS backend_build
ENV RUSTFLAGS="-C target-cpu=native"

WORKDIR /src
ADD Cargo.toml Cargo.lock .env .
ADD noodle-server noodle-server
ADD noodle-client noodle-client
ADD libnoodle libnoodle

RUN apk add gcc
RUN apk add musl-dev
WORKDIR noodle-server
RUN cargo build --release

FROM alpine:latest AS frontend_build
RUN apk add nodejs npm
WORKDIR /src
ADD noodle-client/temp-vue-ui .
RUN npm install
RUN npm run build

FROM nginx:alpine3.22
COPY --from=backend_build /src/target/release/noodle-server /bin/noodle-server
COPY --from=backend_build /src/noodle-server/docker-entrypoint.sh /bin/entrypoint
COPY --from=backend_build /src/.env .

RUN chmod u+x /bin/noodle-server
RUN chmod u+x /bin/entrypoint
COPY --from=frontend_build /src/dist /etc/nginx/html
ADD nginx.conf /etc/nginx/nginx.conf
ENTRYPOINT /bin/entrypoint
