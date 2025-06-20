#!/bin/bash

# AI-Playground Build Script
# This script builds and optionally runs the AI-Playground project

set -e  # Exit on any error

echo "=== AI-Playground Build Script ==="
echo "Building AI-Playground project..."

# Create build directory if it doesn't exist
if [ ! -d "build" ]; then
    echo "Creating build directory..."
    mkdir build
fi

# Navigate to build directory
cd build

# Configure with CMake
echo "Configuring with CMake..."
cmake ..

# Build the project
echo "Building project..."
make

echo "Build completed successfully!"
echo "Executable location: $(pwd)/bin/ai-playground"

# Check if user wants to run the application
if [ "$1" = "--run" ] || [ "$1" = "-r" ]; then
    echo "Running AI-Playground..."
    echo "Press Ctrl+C to exit"
    ./bin/ai-playground
elif [ "$1" = "--demo" ] || [ "$1" = "-d" ]; then
    echo "Running demo..."
    echo -e "1\n2\n7" | ./bin/ai-playground
else
    echo ""
    echo "To run the application:"
    echo "  ./build.sh --run"
    echo "  or"
    echo "  cd build && ./bin/ai-playground"
    echo ""
    echo "To run a quick demo:"
    echo "  ./build.sh --demo"
fi 