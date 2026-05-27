#!/bin/bash
# Bash script to build and auto-rename executable
# Usage: ./rename_build.sh

echo "Building Rust cheat with randomized name..."

# Build and capture output
output=$(cargo build --release 2>&1)

# Extract random name from build warnings
random_name=$(echo "$output" | grep "Randomized process name:" | sed 's/.*Randomized process name: //')

if [ -n "$random_name" ]; then
    echo "Generated name: $random_name"
    
    source_path="target/release/rust-game-cheat.exe"
    dest_path="target/release/$random_name"
    
    if [ -f "$source_path" ]; then
        mv "$source_path" "$dest_path"
        echo "✓ Renamed to: $dest_path"
        echo ""
        echo "Run with: ./$dest_path"
    else
        echo "✗ Build failed or executable not found"
        exit 1
    fi
else
    echo "✗ Could not extract random name from build output"
    exit 1
fi
