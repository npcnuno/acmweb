

# Use the official Rust Alpine image which includes Cargo (Rust toolchain)
FROM rust:1.70-alpine

# Install Node.js, npm, musl-dev (for linking), protobuf-dev (if needed), and curl
RUN apk add --no-cache nodejs npm musl-dev protobuf-dev curl

# Install wasm-pack globally using npm so that it's available for your WASM build
RUN npm install -g wasm-pack

# Set working directory for the app
WORKDIR /frontend

# Copy package files and install dependencies
COPY package*.json ./
RUN npm install

# Copy the entire source code into the container
COPY . .

# Run the build script; this calls "npm run wasm" (which builds your wasm module and runs modulize)
# and then builds your SvelteKit app with Vite.
RUN npm run build

# Expose the port your SvelteKit app will use (e.g. 3000)
EXPOSE 3000

# Start the production server. If you're using adapter‑node, this should launch your built server.
CMD ["npm","run","dev"]



