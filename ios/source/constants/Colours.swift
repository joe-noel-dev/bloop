import SwiftUI

struct Colours {
    static let theme1 = Color("theme-1")
    static let theme2 = Color("theme-2")
    static let theme3 = Color("theme-3")
    static let theme4 = Color("theme-4")
    static let theme5 = Color("theme-5")

    static let selected = theme3
    static let playing = theme4
    static let background = theme1

    static let theme1Text = Color("theme-1-text")
    static let theme2Text = Color("theme-2-text")
    static let theme3Text = Color("theme-3-text")
    static let theme4Text = Color("theme-4-text")
    static let theme5Text = Color("theme-5-text")

    static let neutral0 = Color("neutral-0")
    static let neutral1 = Color("neutral-1")
    static let neutral2 = Color("neutral-2")
    static let neutral3 = Color("neutral-3")
    static let neutral4 = Color("neutral-4")
    static let neutral5 = Color("neutral-5")
    static let neutral6 = Color("neutral-6")
    static let neutral7 = Color("neutral-7")
    static let neutral8 = Color("neutral-8")

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
