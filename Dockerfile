FROM rust:1.92 AS builder
ENV APP_NAME=clinic_core
WORKDIR /usr/src/clinic_booking_api
COPY . .
RUN cargo install --path clinic_core

FROM debian:bookworm-slim
ENV APP_NAME=clinic_core
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/${APP_NAME} /usr/local/bin/${APP_NAME}
EXPOSE 8080
CMD ["clinic_core"]