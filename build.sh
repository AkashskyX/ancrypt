#!/bin/bash

# Define the binary name and target directory
BINARY_NAME="ancrypt"  # Replace with your actual binary name
TARGET_DIR="/usr/local/bin"

# Build the project in release mode
echo "Building the project in release mode..."
cargo build --release

# Check if build was successful
if [ $? -eq 0 ]; then
    # Copy the binary to the target directory
    echo "Copying the binary to ${TARGET_DIR}..."
    cp target/release/${BINARY_NAME} ${TARGET_DIR}/

    # Set execute permissions on the binary
    chmod +x ${TARGET_DIR}/${BINARY_NAME}

    echo "Build and installation completed successfully."
else
    echo "Build failed. Please check the error messages."
fi
