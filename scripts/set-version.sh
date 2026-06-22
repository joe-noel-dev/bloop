#!/usr/bin/env bash

set -e

# Function to print usage
usage() {
    echo "Usage: $0 <version>"
    echo "Example: $0 1.2.3"
    echo "Version should follow semantic versioning format (M.m.p)"
    exit 1
}

# Function to validate version format
validate_version() {
    local version=$1
    if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "Error: Version '$version' does not follow semantic versioning format (M.m.p)"
        exit 1
    fi
}

# Function to update Cargo.toml
update_cargo_toml() {
    local version=$1
    local cargo_file="./core/Cargo.toml"
    
    if [ ! -f "$cargo_file" ]; then
        echo "Error: $cargo_file not found"
        exit 1
    fi
    
    # Update version in Cargo.toml
    sed -i '' "s/^version = \".*\"/version = \"$version\"/" "$cargo_file"
    echo "Updated version in $cargo_file to $version"
}

# Function to update Cargo.lock
update_cargo_lock() {
    local version=$1
    
    echo "Updating Cargo.lock to reflect version change..."
    
    # Use cargo update to refresh the lock file with the new version
    # This will update the lock file to reflect the version change in Cargo.toml
    (cd core && cargo update --workspace)
    
    echo "Cargo.lock updated successfully"
}

# Function to update package.json files
update_package_json() {
    local version=$1
    
    # Find all package.json files and update them
    find . -name "package.json" -type f | while read -r package_file; do
        # Skip node_modules directories
        if [[ "$package_file" == *"/node_modules/"* ]]; then
            continue
        fi
        
        # Check if the file has a version field
        if grep -q '"version"' "$package_file"; then
            # Use sed to update the version field
            sed -i '' 's/"version": *"[^"]*"/"version": "'"$version"'"/' "$package_file"
            echo "Updated version in $package_file to $version"
        fi
    done
}

# Function to update Android version
update_android_version() {
    local version=$1
    local gradle_file="./android/app/build.gradle.kts"

    if [ ! -f "$gradle_file" ]; then
        echo "Warning: $gradle_file not found, skipping Android version update"
        return
    fi

    # Compute versionCode from semver: MAJOR * 10000 + MINOR * 100 + PATCH
    local major minor patch version_code
    IFS='.' read -r major minor patch <<< "$version"
    version_code=$(( major * 10000 + minor * 100 + patch ))

    sed -i '' "s/versionCode = [0-9]*/versionCode = $version_code/" "$gradle_file"
    sed -i '' "s/versionName = \"[^\"]*\"/versionName = \"$version\"/" "$gradle_file"
    echo "Updated Android version in $gradle_file: versionCode=$version_code, versionName=$version"
}

# Function to run iOS version script
run_ios_version_script() {
    local ios_script="./ios/version.sh"
    
    if [ ! -f "$ios_script" ]; then
        echo "Warning: $ios_script not found, skipping iOS version update"
        return
    fi
    
    echo "Running iOS version script..."
    (cd ios && bash version.sh)
    echo "iOS version updated successfully"
}

# Main script execution
main() {
    # Check arguments
    if [ $# -ne 1 ]; then
        usage
    fi
    
    local version=$1
    
    echo "Setting version to: $version"
    
    # Validate version format
    validate_version "$version"
    
    # Ensure we're in the project root
    if [ ! -f "./core/Cargo.toml" ]; then
        echo "Error: Must run from project root (core/Cargo.toml not found)"
        exit 1
    fi
    
    # Update Cargo.toml
    update_cargo_toml "$version"
    
    # Update Cargo.lock
    update_cargo_lock "$version"
    
    # Update Android version
    update_android_version "$version"

    # Run iOS version script
    run_ios_version_script
    
    # Update package.json files
    update_package_json "$version"
    
    echo "Version set to v$version successfully!"
}

# Run main function with all arguments
main "$@"

