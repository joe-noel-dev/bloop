#!/bin/bash

# Exit on any error
set -e

if [ ! -d "pb_data" ]; then
    echo "pb_data directory does not exist. Nothing to back up."
    exit 1
fi

rm -rf pb_data_backup
cp -r pb_data pb_data_backup
echo "Backup of pb_data created at pb_data_backup"