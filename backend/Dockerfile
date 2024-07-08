FROM rust
WORKDIR /usr/src/backend
COPY ./ /usr/src/backend/
RUN cargo build --release
EXPOSE 8000
ENV ASSIGN_LOG=info
CMD ["./target/release/IntrastekApi"]
