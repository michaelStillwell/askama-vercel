FROM rust:latest as builder
ARG APP_NAME

COPY . .
RUN cargo build --locked --release
CMD ["/target/release/vercel-rust-test"]
