import Foundation

struct Layout {
    static let cornerRadiusSmall = 2.0
    static let corderRadiusMedium = 4.0
    static let cornerRadiusLarge = 8.0
    static let cornerRadiusXLarge = 12.0

    static let touchTarget = 48.0
    
    // Icon sizes
    static let iconSmall = 24.0
    static let iconMedium = 48.0
    static let iconLarge = 72.0
    
    // Border widths
    static let borderThin = 1.0
    static let borderMedium = 2.0

    static func units(_ count: CGFloat) -> CGFloat {
        count * 8.0
    }

}
