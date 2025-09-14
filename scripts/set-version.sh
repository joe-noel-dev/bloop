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

# Function to check if tag exists
check_tag_exists() {
    local version=$1
    local tag="v$version"
    
    if git tag -l | grep -q "^$tag$"; then
        echo "Error: Tag '$tag' already exists"
        exit 1
    fi
}

# Function to update Cargo.toml
update_cargo_toml() {
    local version=$1
    local cargo_file="./Cargo.toml"
    
    if [ ! -f "$cargo_file" ]; then
        echo "Error: $cargo_file not found"
        exit 1
    fi
    
    # Update version in Cargo.toml
    sed -i '' "s/^version = \".*\"/version = \"$version\"/" "$cargo_file"
    echo "Updated version in $cargo_file to $version"
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

# Function to commit and tag changes
commit_and_tag() {
    local version=$1
    local tag="v$version"
    local commit_message="Bump version to $tag"
    
    # Add all changes
    git add .
    
    # Check if there are any changes to commit
    if git diff --staged --quiet; then
        echo "No changes to commit"
        return
    fi
    
    # Commit changes
    git commit -m "$commit_message"
    echo "Committed changes with message: $commit_message"
    
    # Create tag
    git tag "$tag"
    echo "Created tag: $tag"
}

# Function to push to main
push_to_main() {
    local version=$1
    local tag="v$version"
    local current_branch=$(git branch --show-current)
    
    echo "Current branch: $current_branch"
    
    # If we're not on main, we need to merge or switch
    if [ "$current_branch" != "main" ]; then
        echo "Switching to main branch..."
        git checkout main
        echo "Merging changes from $current_branch..."
        git merge "$current_branch" --no-ff -m "Merge version bump $tag from $current_branch"
    fi
    
    # Push commit and tag to main
    echo "Pushing commit and tag to main..."
    git push origin main
    git push origin "$tag"
    echo "Successfully pushed commit and tag to main"
    
    # Switch back to original branch if we changed
    if [ "$current_branch" != "main" ]; then
        echo "Switching back to $current_branch..."
        git checkout "$current_branch"
    fi
}

# Main script execution
main() {
    # Check that we're on the main branch
    local current_branch=$(git branch --show-current)
    if [ "$current_branch" != "main" ]; then
        echo "Error: This script must be run on the main branch"
        echo "Current branch: $current_branch"
        echo "Please switch to main branch first: git checkout main"
        exit 1
    fi
    
    # Check arguments
    if [ $# -ne 1 ]; then
        usage
    fi
    
    local version=$1
    
    echo "Setting version to: $version"
    
    # Validate version format
    validate_version "$version"
    
    # Check if tag already exists
    check_tag_exists "$version"
    
    # Ensure we're in the project root
    if [ ! -f "./Cargo.toml" ]; then
        echo "Error: Must run from project root (Cargo.toml not found)"
        exit 1
    fi
    
    # Update Cargo.toml
    update_cargo_toml "$version"
    
    # Run iOS version script
    run_ios_version_script
    
    # Update package.json files
    update_package_json "$version"
    
    # Commit and tag changes
    commit_and_tag "$version"
    
    # Push to main
    push_to_main "$version"
    
    echo "Version bump to v$version completed successfully!"
}

# Run main function with all arguments
main "$@"

