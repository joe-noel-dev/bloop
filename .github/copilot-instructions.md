# Project Guidelines

## Package Managers
**For the editor (`/editor` directory): ALWAYS use yarn, never npm.** The editor uses yarn as the package manager. Use yarn commands:
- `yarn install` (not `npm install`)
- `yarn start` (not `npm start`) 
- `yarn build` (not `npm run build`)
- `yarn add <package>` (not `npm install <package>`)

**For other parts of the project:** Use the appropriate package manager for that language/platform (e.g., Cargo for Rust, CocoaPods/Swift Package Manager for iOS)

## Code Style
Prefer self-documenting code over comments. Add comments only when they add context that isn't already clear from the code itself. Avoid excessive commenting.

## Testing
Suggest tests where there isn't coverage.

## Code Quality  
Suggest removing duplication where it could be extracted.

## Theme System
Use the centralized theme system (`src/theme/`) instead of hardcoded constants. Import theme tokens like `shadows`, `transitions`, `colors`, and `spacing` rather than using hardcoded values like `'0 4px 12px rgba(0, 0, 0, 0.15)'` or `'all 0.2s ease'`.
