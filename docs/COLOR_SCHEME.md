# Unified Color Scheme

This document describes the unified color scheme used across all Bloop platforms (Editor, iOS app, and Core UI).

## Overview

The Bloop color scheme is centralized and synchronized across all platforms. The **Editor theme system** (`editor/src/theme/tokens.ts`) serves as the source of truth for all color values.

## Color Palette

### Primary Color
- **Color**: `#ffab91` (warm coral/peach)
- **Usage**: Main brand color, primary actions, highlights
- **Platforms**:
  - Editor: `colors.primary.main`
  - iOS: `Colours.theme1`
  - Core: `theme::PRIMARY`

### Theme Colors (Extended Palette)

The theme provides 5 additional palette colors for variety and semantic meaning:

| Color | Hex | Description | Editor | iOS | Core |
|-------|-----|-------------|--------|-----|------|
| Theme 1 | `#ffab91` | Coral/Peach (primary) | `colors.primary.main` | `theme1` | `PRIMARY` |
| Theme 2 | `#bcd8c1` | Mint Green | `colors.palette.1` | `theme2` | `palette::COLOR_1` |
| Theme 3 | `#388697` | Teal | `colors.palette.3` | `theme3` | `palette::COLOR_3` |
| Theme 4 | `#cc2936` | Red | `colors.palette.4` | `theme4` | `palette::COLOR_4` |
| Theme 5 | `#a93f55` | Burgundy | `colors.palette.5` | `theme5` | `palette::COLOR_5` |

### Neutral Colors

A grayscale palette with a subtle blue tint (hsl(240, 5%, X%)):

| Level | HSL | Hex | Description | Editor | iOS | Core |
|-------|-----|-----|-------------|--------|-----|------|
| 0 | - | `#ffffff` | White | `colors.neutral.0` | `neutral0` | `neutral::N0` |
| 1 | hsl(240, 5%, 88.5%) | `#e0e0e3` | Lightest gray | `colors.neutral.1` | `neutral1` | `neutral::N1` |
| 2 | hsl(240, 5%, 75%) | `#bcbcc2` | Light gray | `colors.neutral.2` | `neutral2` | `neutral::N2` |
| 3 | hsl(240, 5%, 67.5%) | `#a7a7b0` | Medium-light gray | `colors.neutral.3` | `neutral3` | `neutral::N3` |
| 4 | hsl(240, 5%, 50%) | `#797985` | Medium gray | `colors.neutral.4` | `neutral4` | `neutral::N4` |
| 5 | hsl(240, 5%, 37.5%) | `#5a5a64` | Medium-dark gray | `colors.neutral.5` | `neutral5` | `neutral::N5` |
| 6 | hsl(240, 5%, 25%) | `#3c3c42` | Dark gray | `colors.neutral.6` | `neutral6` | `neutral::N6` |
| 7 | hsl(240, 5%, 12.5%) | `#1e1e21` | Darkest gray | `colors.neutral.7` | `neutral7` | `neutral::N7` |
| 8 | - | `#000000` | Black | `colors.neutral.8` | `neutral8` | `neutral::N8` |

### Background Colors

| Color | Hex | Description | Editor | iOS | Core |
|-------|-----|-------------|--------|-----|------|
| Light | `#ffffff` | Light mode background | `colors.background` | `backgroundLight` | - |
| Dark | `#1f1f20` | Dark mode background | `colors.backgroundDark` | `backgroundDark` | - |

## Platform-Specific Usage

### Editor (TypeScript/React)

The Editor uses the centralized theme system in `editor/src/theme/tokens.ts`:

```typescript
import { colors } from '../theme';

// Access colors
const primaryColor = colors.primary.main;  // #ffab91
const paletteColor = colors.palette[1];     // #bcd8c1
const neutralColor = colors.neutral[4];     // #797985
```

CSS variables are also available in `editor/src/index.css`:

```css
.my-element {
  background: var(--primary);
  color: var(--neutral-7);
}
```

### iOS App (Swift/SwiftUI)

iOS colors are defined in `ios/source/constants/Colours.swift` and backed by Xcode color assets:

```swift
import SwiftUI

// Use predefined colors
Text("Hello")
  .foregroundColor(Colours.theme1)        // Primary color
  .background(Colours.backgroundLight)    // White background
  
// Use neutral colors
Rectangle()
  .fill(Colours.neutral4)                 // Medium gray
```

Color assets are located in `ios/source/constants/Colours.xcassets/`.

### Core UI (Rust/Iced)

Core UI uses the theme module in `core/src/ui/theme.rs`:

```rust
use crate::ui::theme;

// Use color constants
let primary_color = theme::PRIMARY;
let palette_color = theme::palette::COLOR_1;
let neutral_color = theme::neutral::N4;
```

The unified theme is applied via `create_bloop_theme()` in the view module.

## Color Conversion Guide

When adding new colors or updating existing ones:

1. **Define in Editor first**: Add/modify colors in `editor/src/theme/tokens.ts`
2. **Convert to iOS format**:
   - Convert hex to RGB values (0-255)
   - Create JSON in Xcode color asset format
   - Use sRGB color space for compatibility
3. **Add to Core Rust**:
   - Convert RGB to float values (0.0-1.0)
   - Define as `Color` constant in `core/src/ui/theme.rs`

### Conversion Formula

```
// Hex to RGB
R = parseInt(hex.substring(1, 3), 16)  // 0-255
G = parseInt(hex.substring(3, 5), 16)  // 0-255
B = parseInt(hex.substring(5, 7), 16)  // 0-255

// RGB to iOS (hex format)
"0xRR", "0xGG", "0xBB"

// RGB to Core (float format)
r: R / 255.0  // 0.0-1.0
g: G / 255.0  // 0.0-1.0
b: B / 255.0  // 0.0-1.0
```

## Maintenance

To maintain color consistency:

1. **Always update all platforms** when changing colors
2. **Use the Editor theme as source of truth**
3. **Test colors on all platforms** after changes
4. **Document semantic meanings** for new colors
5. **Keep neutral scale consistent** across light/dark modes

## Migration History

- **October 2025**: Unified color scheme implemented
  - iOS colors updated from bright primaries (green, red, purple, orange, yellow) to match Editor palette
  - Core UI theme changed from Moonfly to custom Bloop theme
  - All neutral colors synchronized using HSL values
  - Background colors unified across platforms
