# Builder stage
FROM rust:1.81.0 as builder
WORKDIR /usr/src/bot-ukyo
COPY . .

RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

# Install OpenSSL and its dependencies
RUN apt-get update && \
  apt-get install -y libssl3 ca-certificates && \
  rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage to /usr/local/bin
COPY --from=builder /usr/src/bot-ukyo/target/release/bot-ukyo /usr/local/bin/bot-ukyo

CMD ["/usr/local/bin/bot-ukyo"]
