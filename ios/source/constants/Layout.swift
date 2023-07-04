import Foundation

struct Layout {
    static let cornerRadiusSmall = 2.0
    static let corderRadiusMedium = 4.0
    static let cornerRadiusLarge = 8.0

    static let touchTarget = 48.0

    static func units(_ count: CGFloat) -> CGFloat {
        count * 8.0
    }

}
