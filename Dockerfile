ARG APP_NAME=clinic_core

FROM rust:1.96.0-slim AS builder
ARG APP_NAME
WORKDIR /usr/src/clinic_booking_api
COPY . .
RUN cargo install --path ${APP_NAME}

FROM debian:bookworm-slim
ARG APP_NAME
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/${APP_NAME} /usr/local/bin/${APP_NAME}
EXPOSE 8080
CMD ["clinic_core"]
