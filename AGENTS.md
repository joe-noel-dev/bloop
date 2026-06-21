# Agents

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

Use the centralized Bloop theme system instead of hardcoding colors. See `docs/COLOR_SCHEME.md` for the full color reference and hex values.

### Android (Jetpack Compose)

Colors are defined in `android/app/src/main/java/com/joenoel/bloop/ui/theme/Color.kt`. Use `MaterialTheme.colorScheme.*` in composables rather than hardcoded color literals:

```kotlin
// Good
Text(color = MaterialTheme.colorScheme.primary)
Box(modifier = Modifier.background(MaterialTheme.colorScheme.surfaceVariant))

// Avoid
Text(color = Color(0xFFFFAB91))
Box(modifier = Modifier.background(Color.Gray))
```

### iOS (SwiftUI)

Colors are defined in `ios/source/constants/Colours.swift`. Use `Colours.theme1`, `Colours.neutral4`, `Colours.backgroundDark`, etc. rather than hardcoded hex or RGB values.

### Editor (TypeScript/React)

Use `colors` from `editor/src/theme/tokens.ts` or CSS variables from `editor/src/index.css`. Import theme tokens like `shadows`, `transitions`, `colors`, and `spacing` rather than hardcoded values like `'0 4px 12px rgba(0, 0, 0, 0.15)'` or `'all 0.2s ease'`.

### Core (Rust/Iced)

Use constants from `core/src/ui/theme.rs` (e.g., `theme::PRIMARY`, `theme::neutral::N4`) rather than inline `Color` literals.
