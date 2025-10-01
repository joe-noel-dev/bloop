#!/bin/bash

# Exit on any error
set -e

if [ -d "pb_data" ]; then
    echo "Removing pb_data directory..."
    rm -rf pb_data
    echo "pb_data directory removed."
else
    echo "pb_data directory does not exist. Nothing to clean."
fi
