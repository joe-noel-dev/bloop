# Testing Infrastructure

This directory contains the testing infrastructure for the bloop project, including support for both mock and real PocketBase backends.

## Overview

The test infrastructure supports two backend types:

1. **Mocketbase** - A mock HTTP server that simulates PocketBase API responses using wiremock
2. **LocalPocketbase** - A real PocketBase instance running locally for integration testing

## Components

### Mock Backend (`mocketbase.rs`)
- Uses wiremock to simulate PocketBase API responses
- Fast and reliable for unit testing
- No external dependencies
- Limited to predefined response patterns

### Local PocketBase Backend (`local_pocketbase.rs`)
- Downloads and runs a real PocketBase binary
- Loads the actual schema from `backend/pb_schema.json`
- Supports creating real test users
- Full integration testing capabilities

### Integration Fixtures
- `IntegrationFixture` - Original fixture using mock backend
- `EnhancedIntegrationFixture` - New fixture supporting both backend types

## Usage

### Basic Mock Testing (Existing Pattern)
```rust
use crate::common::{IntegrationFixture, MockUser};

#[tokio::test]
async fn test_with_mock() {
    let mut fixture = IntegrationFixture::new().await;
    
    let user = MockUser::new("user-id", "test@example.com", "password", "Test User");
    fixture.mocketbase().add_user(user.clone()).await;
    
    // Test your application logic...
}
```

### Enhanced Testing with Real PocketBase
```rust
use crate::common::{EnhancedIntegrationFixture, TestUser};

#[tokio::test]
async fn test_with_real_pocketbase() {
    let mut fixture = EnhancedIntegrationFixture::new_with_real_pocketbase().await;
    
    // The server is automatically started and schema loaded
    // You can add users programmatically
    let user = TestUser::new("user-id", "test@example.com", "password", "Test User");
    fixture.local_pocketbase().add_user(user).await.unwrap();
    
    // Test your application logic with real backend...
}
```

### Backend-Agnostic Testing
```rust
use crate::common::EnhancedIntegrationFixture;

#[tokio::test]
async fn test_with_either_backend() {
    // Use mock by default
    let mut fixture = EnhancedIntegrationFixture::new().await;
    
    // Or explicitly use real PocketBase
    // let mut fixture = EnhancedIntegrationFixture::new_with_real_pocketbase().await;
    
    // Unified interface for adding users
    fixture.add_test_user("test@example.com", "password", "Test User").await.unwrap();
    
    // Test your application logic...
}
```

## LocalPocketbase Features

### Automatic Binary Management
- Downloads PocketBase binary automatically if not present
- Uses version 0.27.1 (configurable)
- Supports multiple platforms (Linux, macOS, Windows)
- Binary is downloaded to a temporary directory (not committed to repo)

### Schema Loading
- Automatically loads schema from `backend/pb_schema.json`
- Creates collections that don't exist
- Skips system collections (they exist by default)
- Handles collection creation errors gracefully

### Test User Management
- Programmatic user creation via API
- Proper password hashing and validation
- Email verification handling
- Supports custom user attributes

### Lifecycle Management
- Automatic startup and shutdown
- Finds free ports automatically
- Health check verification
- Proper cleanup on test completion

## Configuration

### Schema File Location
The schema file must be located at `backend/pb_schema.json` relative to the project root. This file contains the PocketBase collection definitions exported from your PocketBase admin interface.

### PocketBase Version
The PocketBase version is currently hardcoded to 0.27.1 in `local_pocketbase.rs`. You can modify the `PB_VERSION` constant to use a different version if needed.

### Port Configuration
LocalPocketbase automatically finds free ports to avoid conflicts. You can access the URL using `local_pocketbase.url()`.

## Troubleshooting

### Schema Loading Issues
If schema loading fails:
1. Verify `backend/pb_schema.json` exists and is valid JSON
2. Check that the PocketBase server started successfully
3. Review console output for specific error messages

### User Creation Issues
If user creation fails:
1. Ensure the "users" collection exists in your schema
2. Check field requirements (email format, password length, etc.)
3. Verify the server is accessible at the reported URL

### Binary Download Issues
If PocketBase binary download fails:
1. Check internet connectivity for GitHub releases
2. Verify the platform detection is correct
3. Check disk space in temporary directory

## Examples

See the test files for working examples:
- `local_pocketbase_tests.rs` - Basic LocalPocketbase functionality
- `local_pocketbase_integration_test.rs` - Integration testing examples
- `login_tests.rs` - Original mock-based testing pattern

## Future Improvements

Potential enhancements:
- Admin user authentication for collection management
- Database seeding from fixtures
- Custom PocketBase configuration files
- Integration with CI/CD pipelines
- Performance benchmarking against mock