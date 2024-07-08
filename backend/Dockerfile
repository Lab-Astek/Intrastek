FROM rust
WORKDIR /usr/src/backend
COPY ./ /usr/src/backend/
RUN cargo build --release
ENV ASSIGN_LOG=debug
CMD ["./target/release/IntrastekApi"]
