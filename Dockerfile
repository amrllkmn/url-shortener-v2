# Use the official Rust image as the base image
FROM rust:latest as builder

# Create a new directory for our application
WORKDIR /usr/src/app

# Copy the application files to the container
COPY . .

# Build the application
RUN cargo build --release

# Use a new image as the base image
FROM debian:buster-slim

# Install OpenSSL and create a new directory for our application
RUN apt-get update && apt-get install -y openssl curl && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/app

# Copy the binary from the previous stage to the container
COPY --from=builder /usr/src/app/target/release/url-shortener-v2 .

# Expose the port that the application listens on
EXPOSE 8080

# Start the application
CMD ["./url-shortener-v2"]