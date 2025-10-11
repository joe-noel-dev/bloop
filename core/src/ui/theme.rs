use iced::{Color, Theme};

/// Creates a custom theme matching the unified color scheme
/// Colors are synchronized with:
/// - editor/src/theme/tokens.ts
/// - ios/source/constants/Colours.swift
pub fn create_bloop_theme() -> Theme {
    // Create a custom dark theme with our primary color
    let mut palette = Theme::TokyoNightStorm.palette();
    
    // Override the primary color to use our coral/peach
    palette.primary = PRIMARY;
    
    // Override the background to use our dark theme background (#1f1f20)
    palette.background = BACKGROUND_DARK;
    
    Theme::custom("Bloop".to_string(), palette)
}

/// Primary color - matches Editor primary.main (#ffab91)
#[allow(dead_code)]
pub const PRIMARY: Color = Color {
    r: 1.0,      // 255/255
    g: 0.671,    // 171/255
    b: 0.569,    // 145/255
    a: 1.0,
};

/// Dark background color - matches Editor backgroundDark (#1f1f20)
pub const BACKGROUND_DARK: Color = Color {
    r: 0.122,    // 31/255
    g: 0.122,    // 31/255
    b: 0.125,    // 32/255
    a: 1.0,
};

/// Palette colors matching Editor theme
#[allow(dead_code)]
pub mod palette {
    use iced::Color;
    
    /// Palette 1 - mint green (#bcd8c1)
    pub const COLOR_1: Color = Color {
        r: 0.737,    // 188/255
        g: 0.847,    // 216/255
        b: 0.757,    // 193/255
        a: 1.0,
    };
    
    /// Palette 3 - teal (#388697)
    pub const COLOR_3: Color = Color {
        r: 0.220,    // 56/255
        g: 0.525,    // 134/255
        b: 0.592,    // 151/255
        a: 1.0,
    };
    
    /// Palette 4 - red (#cc2936)
    pub const COLOR_4: Color = Color {
        r: 0.800,    // 204/255
        g: 0.161,    // 41/255
        b: 0.212,    // 54/255
        a: 1.0,
    };
    
    /// Palette 5 - burgundy (#a93f55)
    pub const COLOR_5: Color = Color {
        r: 0.663,    // 169/255
        g: 0.247,    // 63/255
        b: 0.333,    // 85/255
        a: 1.0,
    };
}

/// Neutral colors matching Editor neutral scale (hsl(240, 5%, X%))
#[allow(dead_code)]
pub mod neutral {
    use iced::Color;
    
    /// Neutral 0 - white
    pub const N0: Color = Color::WHITE;
    
    /// Neutral 1 - hsl(240, 5%, 88.5%)
    pub const N1: Color = Color {
        r: 0.878,    // 224/255
        g: 0.878,    // 224/255
        b: 0.890,    // 227/255
        a: 1.0,
    };
    
    /// Neutral 2 - hsl(240, 5%, 75%)
    pub const N2: Color = Color {
        r: 0.737,    // 188/255
        g: 0.737,    // 188/255
        b: 0.761,    // 194/255
        a: 1.0,
    };
    
    /// Neutral 3 - hsl(240, 5%, 67.5%)
    pub const N3: Color = Color {
        r: 0.655,    // 167/255
        g: 0.655,    // 167/255
        b: 0.690,    // 176/255
        a: 1.0,
    };
    
    /// Neutral 4 - hsl(240, 5%, 50%)
    pub const N4: Color = Color {
        r: 0.475,    // 121/255
        g: 0.475,    // 121/255
        b: 0.522,    // 133/255
        a: 1.0,
    };
    
    /// Neutral 5 - hsl(240, 5%, 37.5%)
    pub const N5: Color = Color {
        r: 0.353,    // 90/255
        g: 0.353,    // 90/255
        b: 0.392,    // 100/255
        a: 1.0,
    };
    
    /// Neutral 6 - hsl(240, 5%, 25%)
    pub const N6: Color = Color {
        r: 0.235,    // 60/255
        g: 0.235,    // 60/255
        b: 0.259,    // 66/255
        a: 1.0,
    };
    
    /// Neutral 7 - hsl(240, 5%, 12.5%)
    pub const N7: Color = Color {
        r: 0.118,    // 30/255
        g: 0.118,    // 30/255
        b: 0.129,    // 33/255
        a: 1.0,
    };
    
    /// Neutral 8 - black
    pub const N8: Color = Color::BLACK;
}
