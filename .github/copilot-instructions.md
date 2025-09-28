Prefer self-documenting code over comments. Add comments only when they add context that isn't already clear from the code itself.

Suggest tests where there isn't coverage.

Suggest removing duplication where it could be extracted.

Use the centralized theme system (`src/theme/`) instead of hardcoded constants. Import theme tokens like `shadows`, `transitions`, `colors`, and `spacing` rather than using hardcoded values like `'0 4px 12px rgba(0, 0, 0, 0.15)'` or `'all 0.2s ease'`.