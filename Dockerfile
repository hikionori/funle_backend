# Stage 1: Build the Rust app
FROM rust:latest as rust-builder
WORKDIR /app/api

# Set nightly toolchain
RUN rustup default nightly

# Copy all files except the admin folder
COPY . .
RUN rm -rf admin

# Build the Rust app
RUN cargo build --release

# Stage 2: Build the Next.js app
FROM node:slim as nextjs-builder
WORKDIR /app/admin

# Install Yarn globally
RUN npm install -g yarn --force

# Copy the admin folder
COPY admin .

# Install dependencies using Yarn
RUN yarn install --frozen-lockfile

# Build the Next.js app
RUN yarn build

# Stage 3: Final image
FROM alpine:latest
WORKDIR /app

# Copy the Rust app binary
COPY --from=rust-builder /app/api/target/release/math_backend ./api/

# Copy the Next.js app build files
COPY --from=nextjs-builder /app/admin/.next ./.next
COPY --from=nextjs-builder /app/admin/public ./public

# Expose the port that the Next.js app will run on
EXPOSE 3000

EXPOSE 8000

ENV MONGO_URL=mongodb://root:root@127.0.0.1:27017

# Set the command to run the apps
CMD sh -c "cd api && cargo run --release & cd .. & yarn start"
