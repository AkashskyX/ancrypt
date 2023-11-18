#!/bin/bash

# Initialize the environment variable to 4100
POOL_SIZE=4100

# Set the environment variable
export POOL_SIZE

# Define the maximum number of retries
MAX_RETRIES=10

# Loop to run the Rust program with decreasing POOL_SIZE
for (( i=0; i<$MAX_RETRIES; i++ )); do
    echo "Running with POOL_SIZE=$POOL_SIZE"
    cargo run

    # Check if the program was successful (exit code 0)
    if [ $? -eq 0 ]; then
        echo "Rust program succeeded."
        break
    else
        echo "Rust program failed. Decreasing POOL_SIZE."
        ((POOL_SIZE--))
        export POOL_SIZE
    fi
done

# Print the final value of POOL_SIZE
echo "Final POOL_SIZE: $POOL_SIZE"
# Print the final value of POOL_SIZE
echo "Exporting: $POOL_SIZE"

# Export the final POOL_SIZE
export POOL_SIZE=$POOL_SIZE 
