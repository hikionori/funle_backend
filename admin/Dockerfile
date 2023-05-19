# Use the official Node.js image as the base image
FROM node:latest

# Install Yarn globally
RUN npm install -g yarn --force

# Set the working directory inside the container
WORKDIR /app

# Copy package.json and yarn.lock to the working directory
COPY package*.json yarn.lock ./

# Install dependencies using Yarn
RUN yarn install --frozen-lockfile

# Copy the entire app directory (including .next) to the working directory
COPY . .

# Build the Next.js app
RUN yarn build

# Expose the port that the Next.js app will run on
EXPOSE 3000
# EXPOSE 8080

# Set the command to start the Next.js app
CMD ["yarn", "start"]
