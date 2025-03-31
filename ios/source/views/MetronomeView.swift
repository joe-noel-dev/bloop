import SwiftUI

struct MetronomeView: View {
    let dotCount = 4

    var isPlaying: Bool
    var sectionBeat: Double

    var body: some View {
        if isPlaying {
            HStack(spacing: Layout.units(4)) {
                ForEach(0..<dotCount, id: \.self) { beatIndex in
                    RoundedRectangle(cornerRadius: Layout.units(0.5))
                        .fill(isBeat(beatIndex) ? Colours.theme1 : Colours.neutral4)
                        .frame(width: Layout.units(4), height: Layout.units(2))
                        .scaleEffect(isBeat(beatIndex) ? 1.3 : 1.0)
                }
            }
            .padding(.horizontal)
        }
    }

    private func isBeat(_ beatIndex: Int) -> Bool {
        Int(sectionBeat) % dotCount == beatIndex
    }
}

struct MetronomeView_Previews: PreviewProvider {
    static var previews: some View {
        MetronomeView(isPlaying: true, sectionBeat: 1)
            .previewLayout(.fixed(width: 200, height: 50))
    }
}
