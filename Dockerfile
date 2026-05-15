FROM rust:slim AS builder
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk --locked
WORKDIR /app
COPY . .
RUN trunk build --release

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80
