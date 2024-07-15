FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:latest as runner
COPY --from=builder /usr/local/cargo/bin/IntrastekApi /usr/local/bin/IntrastekApi
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
ENV INTRASTEK_LOG=info
CMD ["IntrastekApi"]
