# Define the Rust build environment
FROM rust:1.76 as builder

# Create a working directory
WORKDIR /usr/src/cafard

# Copy over the manifests
COPY ./server/Cargo.toml ./server/Cargo.toml

# Copy the Cargo.lock if you are using one to lock dependencies across builds
#COPY ./server/Cargo.lock ./server/Cargo.lock

# Copy the entire sdk and server directories
COPY ./sdk ./sdk
COPY ./server ./server

# This dummy build is to cache dependencies; it will ensure that future builds are faster if dependencies don't change
RUN cd server && cargo build --release
RUN rm server/src/*.rs

# Copy the actual source code files into the image
COPY ./server/src ./server/src

# Build the application in release mode
RUN cd server && cargo build --release

# Define the final base image
FROM debian:buster-slim

# Install OpenSSL, required for Actix-web and many other Rust applications
#RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the build stage
COPY --from=builder /usr/src/cafard/server/target/release/server /usr/local/bin/server

# Set the startup command to run your binary
CMD ["server"]