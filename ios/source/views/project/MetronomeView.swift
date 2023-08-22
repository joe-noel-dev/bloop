import SwiftUI

struct MetronomeView: View {
    let dotCount = 4
    
    var isPlaying: Bool
    var sectionBeat: Int
    
    var body: some View {
        
        if isPlaying {
            HStack{
                ForEach((0..<dotCount), id: \.self) { beatIndex in
                    Circle()
                        .fill(isBeat(beatIndex) ? Colours.theme1 : Colours.neutral4)
                }
            }
        }
        
    }
    
    private func isBeat(_ beatIndex: Int) -> Bool {
        sectionBeat % dotCount == beatIndex
    }

}

struct MetronomeView_Previews: PreviewProvider {
    static var previews: some View {
        MetronomeView(isPlaying: true, sectionBeat: 1)
            .previewLayout(.fixed(width: 200, height: 50))
    }
}
