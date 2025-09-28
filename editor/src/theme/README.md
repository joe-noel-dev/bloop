# Theme System

This directory contains the centralized theming system for the Bloop editor.

## Overview

The theme system provides consistent design tokens across the application, replacing hardcoded values with centralized constants.

## Usage

### Import theme tokens

```typescript
import { shadows, transitions, colors, spacing } from '../theme';
// or
import { getShadow, getTransition, getColor, getSpacing } from '../theme';
```

### Using in MUI Joy components

```typescript
// Instead of hardcoded values:
sx={{
  boxShadow: '0 4px 12px rgba(0, 0, 0, 0.15)',
  transition: 'all 0.2s ease',
}}

// Use theme tokens:
sx={{
  boxShadow: shadows.hover,
  transition: transitions.fast,
}}
```

### Using utility functions

```typescript
// For dynamic color access:
const primaryColor = getColor('primary.main');
const neutralColor = getColor('neutral.3');

// For spacing calculations:
const padding = getSpacing('md'); // Returns '16px'
```

## Available Tokens

### Colors
- `colors.primary` - Primary color palette (main, light, dark)
- `colors.secondary` - Secondary color palette  
- `colors.neutral` - Neutral colors (0-8 scale)
- `colors.palette` - Extended palette colors

### Shadows
- `shadows.level1/2/3` - Standard elevation shadows
- `shadows.hover/focus/active` - Interactive shadows
- `shadows.elevated/soft` - Special purpose shadows

### Spacing
- `spacing.unit` - Base 8px unit
- `spacing.xs/sm/md/lg/xl` - Common spacing values

### Transitions
- `transitions.fast/normal/slow` - Standard durations
- `transitions.easeOut/easeIn` - Easing curves
- `transitions.transform` - Specific property transitions

### Border Radius
- `radii.sm/md/lg/full` - Border radius values

## CSS Variables

CSS variables are defined in `src/index.css` and aligned with the TypeScript tokens for consistency across the application.

## Migration Guide

When migrating existing components:

1. Import theme tokens at the top of your file
2. Replace hardcoded `rgba()` values with shadow tokens
3. Replace hardcoded transitions with transition tokens  
4. Use TypeScript types for better development experience

### Before:
```typescript
sx={{
  boxShadow: '0 4px 12px rgba(0, 0, 0, 0.15)',
  transition: 'all 0.2s ease',
}}
```

### After:
```typescript
import { shadows, transitions } from '../theme';

sx={{
  boxShadow: shadows.hover,
  transition: transitions.fast,
}}
```