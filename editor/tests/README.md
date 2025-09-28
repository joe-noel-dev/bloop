# Editor Testing Infrastructure

This directory contains the testing infrastructure for the editor component of the bloop project, including support for both mock and real PocketBase backends.

## Overview

The editor testing infrastructure provides:

1. **Unit Tests** - Component and reducer testing using Vitest and React Testing Library
2. **Integration Tests** - Backend integration testing with configurable PocketBase backends
3. **Test Utilities** - Helpers for creating test backends and test data

## Test Types

### Unit Tests
- Located in `src/` alongside the components they test
- Use `.test.ts` or `.test.tsx` extension
- Test individual components and functions in isolation
- Examples: `src/dispatcher/reducer.test.ts`, `src/app/sample/Sample.test.tsx`

### Integration Tests
- Located in `tests/` directory
- Test interactions between components and backend services
- Can be configured to use either mock or real PocketBase backends
- Example: `tests/backend-integration.test.ts`

## Backend Testing

### Test Backend Configuration

The `createTestBackend()` function supports different backend configurations:

```typescript
import { createTestBackend } from './tests/test-backend';

// Use default backend (production PocketBase)
const backend = createTestBackend();

// Use local PocketBase instance
const backend = createTestBackend({
  useLocalPocketbase: true,
  pocketbaseUrl: 'http://127.0.0.1:8090'
});
```

### Environment Variables

Control test behavior with environment variables:

- `VITE_USE_LOCAL_POCKETBASE=true` - Enable integration tests with local PocketBase
- `VITE_LOCAL_POCKETBASE_URL=http://127.0.0.1:8090` - Set local PocketBase URL

### Local PocketBase Integration

To run integration tests against a local PocketBase instance:

1. **Start LocalPocketbase from Rust tests:**
   ```bash
   # From the project root
   cd /path/to/project
   cargo test --test local_pocketbase_tests test_local_pocketbase_startup_and_user_creation
   ```

2. **Or start PocketBase manually:**
   ```bash
   # Download and start PocketBase manually on port 8090
   ./pocketbase serve --http=127.0.0.1:8090
   ```

3. **Run editor integration tests:**
   ```bash
   cd editor
   VITE_USE_LOCAL_POCKETBASE=true yarn test
   ```

## Running Tests

### All Tests
```bash
cd editor
yarn test
```

### Unit Tests Only
```bash
cd editor
yarn test src/
```

### Integration Tests with Local PocketBase
```bash
cd editor
VITE_USE_LOCAL_POCKETBASE=true yarn test tests/
```

### Watch Mode
```bash
cd editor
yarn test:watch
```

### UI Mode
```bash
cd editor
yarn test:ui
```

## Test Utilities

### createTestBackend(config)
Creates a backend instance for testing with configurable PocketBase connection.

### createTestUser(id, email, password, name)
Creates a test user object with the specified properties.

### addTestUser(pocketbaseUrl, user)
Adds a test user directly to a PocketBase instance via API.

## Example Usage

### Basic Component Test
```typescript
import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { MyComponent } from './MyComponent';

describe('MyComponent', () => {
  it('renders correctly', () => {
    render(<MyComponent />);
    expect(screen.getByText('Expected Text')).toBeInTheDocument();
  });
});
```

### Backend Integration Test
```typescript
import { describe, it, expect } from 'vitest';
import { createTestBackend, createTestUser } from '../tests/test-backend';

describe('Backend Integration', () => {
  it('should authenticate user', async () => {
    const backend = createTestBackend({
      useLocalPocketbase: true,
      pocketbaseUrl: 'http://127.0.0.1:8090'
    });
    
    const user = await backend.signIn('test@example.com', 'password');
    expect(user.email).toBe('test@example.com');
  });
});
```

## Coordination with Rust Tests

The editor tests can use the same LocalPocketbase instance as the Rust tests:

1. **Shared Schema**: Both use the same `backend/pb_schema.json` file
2. **Shared Port Range**: Tests find free ports automatically
3. **Shared Test Users**: Both can create and use the same test user format

### Cross-Language Test Workflow

1. Start LocalPocketbase from Rust tests
2. Note the URL printed to console (e.g., `http://127.0.0.1:12345`)
3. Run editor tests with that URL:
   ```bash
   VITE_USE_LOCAL_POCKETBASE=true VITE_LOCAL_POCKETBASE_URL=http://127.0.0.1:12345 yarn test
   ```

## Configuration Files

- `vitest.config.ts` - Vitest configuration
- `tests/setup.ts` - Test environment setup and global mocks
- `tests/test-backend.ts` - Test backend utilities
- `tests/README.md` - This documentation

## Troubleshooting

### Integration Tests Skipped
If integration tests are being skipped, ensure:
- `VITE_USE_LOCAL_POCKETBASE=true` is set
- Local PocketBase is running and accessible
- The correct URL is configured in `VITE_LOCAL_POCKETBASE_URL`

### Authentication Failures
If authentication tests fail:
- Verify the PocketBase instance has the correct schema loaded
- Check that test users can be created via the API
- Ensure the "users" collection exists and is configured correctly

### Connection Errors
If connection to PocketBase fails:
- Verify PocketBase is running on the expected port
- Check firewall and network settings
- Ensure the health endpoint `/api/health` is accessible

## Future Improvements

- Mock PocketBase server for faster unit tests
- Automatic LocalPocketbase startup/teardown for integration tests
- Shared test data fixtures between Rust and TypeScript tests
- Performance benchmarking against production backend