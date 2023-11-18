# Dockerfile

# Use the official Rust image as the base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Rust application files to the container
COPY ./src ./src
COPY .env .
COPY Cargo.toml .


# Build the Rust application with the desired executable name
RUN cargo build --release

# Create a new image to reduce the final image size
FROM debian:bullseye-slim

# Set the working directory inside the container
WORKDIR /usr/local/bin

# Copy only the necessary files from the builder image
COPY --from=builder /app/target/release/tel_bookies .

# Set the entry point for the container
CMD ["./tel_bookies"]
