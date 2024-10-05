ARG APP_NAME="second-chance"
ARG RUST_VERSION="1.81"
ARG ALPINE_VERSION="3.20"
ARG CLI_ARGS=""

FROM rust:${RUST_VERSION} as builder

WORKDIR /usr/src/${APP_NAME}
COPY . .
RUN cargo install --path

FROM alpine:${ALPINE_VERSION}
RUN apk update && apk upgrade
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp

CMD [${APP_NAME}, ${CLI_ARGS}]