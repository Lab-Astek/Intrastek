FROM rust:1 as prisma-cli-builder
WORKDIR /prisma-cli
COPY prisma-cli .
RUN cargo install --locked --path .

FROM rust:1 as api-builder
WORKDIR /api
RUN mkdir -p /api/prisma
RUN mkdir -p /api/src
COPY prisma ./prisma
COPY src ./src
COPY Cargo.* ./
COPY build.rs .
COPY --from=prisma-cli-builder /usr/local/cargo/bin/cargo-prisma /usr/local/bin/cargo-prisma
RUN cargo install --locked --path .


FROM debian:latest as runner
COPY --from=api-builder /usr/local/cargo/bin/IntrastekApi /usr/local/bin/IntrastekApi
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
ENV INTRASTEK_LOG=info
CMD ["IntrastekApi"]
