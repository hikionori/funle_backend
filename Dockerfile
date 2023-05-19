# Stage 1. Build rust app
FROM rust:latest as r-build
WORKDIR /app/api

# set nightly toolchain
RUN rustup default nightly

# copy all files instead of admin folder
COPY . .
RUN rm -rf admin

# build rust app
RUN cargo build --release

# Stage 2. Build admin app
FROM node:slim as n-build
WORKDIR /app/admin

# Install Yarn globally
RUN npm install -g yarn --force

# Copy all inside admin folder
COPY admin .

# Install dependencies using Yarn
RUN yarn install --frozen-lockfile

# Build the Next.js app
RUN yarn build

# Stage 3. Run app on alpine
FROM alpine:latest
WORKDIR /app

# Copy the entire app directory (including .next) to the working directory
COPY --from=r-build /app/api/target/release/math_backend ./math_backend
COPY --from=n-build /app/admin/.next ./.next
COPY --from=n-build /app/admin/public ./public

# Expose the port that the Next.js app will run on
EXPOSE 3000

# Expose the port that the Rust app will run on
EXPOSE 8080

# Env
ENV MONGO_URL=mongodb://root:root@127.0.0.1:27017

# Set the command to start the project
CMD [ "./math_backend", "&&" , "yarn", "start" ]