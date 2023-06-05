# Stage 1: Build the Rust app
FROM rust:latest as rust-builder
WORKDIR /app/api

# Set nightly toolchain
RUN rustup default nightly

# Copy all files except the admin folder
COPY . .
COPY Rocket.toml Rocket.toml
RUN rm -rf admin

# Build the Rust app
RUN cargo build --release

ENV MONGO_URL=mongodb://root:root@localhost:27017/
ENV ROCKET_ENV=production

EXPOSE 8080

# Run the Rust app
CMD ["cargo", "run", "--release"]