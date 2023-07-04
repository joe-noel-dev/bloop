import SwiftUI

struct WaveformView: View {
    var body: some View {
        Colours.neutral2
            .frame(maxWidth: .infinity, maxHeight: .infinity)
            .cornerRadius(Layout.corderRadiusMedium)
    }
}

struct WaveformView_Previews: PreviewProvider {
    static var previews: some View {
        WaveformView()
    }
}
