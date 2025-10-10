import SwiftUI

/// Unified color scheme matching Editor theme system
/// Colors are synchronized with editor/src/theme/tokens.ts
struct Colours {

    // MARK: - Theme Colors (mapped from Editor palette)
    
    /// Primary color - matches Editor primary.main (#ffab91)
    static let theme1 = Color("theme-1")
    
    /// Palette color 1 - matches Editor palette.1 (#bcd8c1)
    static let theme2 = Color("theme-2")
    
    /// Palette color 3 - matches Editor palette.3 (#388697)
    static let theme3 = Color("theme-3")
    
    /// Palette color 4 - matches Editor palette.4 (#cc2936)
    static let theme4 = Color("theme-4")
    
    /// Palette color 5 - matches Editor palette.5 (#a93f55)
    static let theme5 = Color("theme-5")

    // MARK: - Background Colors
    
    /// Light mode background - matches Editor background (white)
    static let backgroundLight = Color("background-light")
    
    /// Dark mode background - matches Editor backgroundDark (hsl(240, 5%, 12.5%))
    static let backgroundDark = Color("background-dark")

    // MARK: - Semantic Colors
    
    static let selected = neutral2
    static let playing = theme1

    // MARK: - Neutral Colors
    /// Neutral scale matches Editor neutral colors (hsl(240, 5%, X%))
    
    static let neutral0 = Color("neutral-0")
    static let neutral1 = Color("neutral-1")
    static let neutral2 = Color("neutral-2")
    static let neutral3 = Color("neutral-3")
    static let neutral4 = Color("neutral-4")
    static let neutral5 = Color("neutral-5")
    static let neutral6 = Color("neutral-6")
    static let neutral7 = Color("neutral-7")
    static let neutral8 = Color("neutral-8")
    
    // Gradients
    static let primaryGradient = LinearGradient(
        colors: [theme1, theme1],
        startPoint: .topLeading,
        endPoint: .bottomTrailing
    )
    
    static let buttonGradient = LinearGradient(
        colors: [theme1, theme1],
        startPoint: .leading,
        endPoint: .trailing
    )
    
    static let disabledGradient = LinearGradient(
        colors: [.gray, .gray],
        startPoint: .leading,
        endPoint: .trailing
    )

    static func neutralText(against contrast: Color) -> Color {
        switch contrast {
        case neutral0, neutral1, neutral2, neutral3, neutral4:
            return .black

        case neutral5, neutral6, neutral7, neutral8:
            return .white

        default:
            return .black
        }

    }
}
