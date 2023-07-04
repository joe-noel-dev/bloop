import SwiftUI

struct ProgressBarStyle {

}

struct ProgressBar: View {
    var progress: Double

    var body: some View {
        GeometryReader { geometry in
            ZStack {
                Rectangle()
                    .fill(.foreground)
                    .frame(width: progress * geometry.size.width, height: geometry.size.height)
            }
        }
    }
}

struct ProgressBar_Previews: PreviewProvider {
    static var previews: some View {
        ProgressBar(progress: 0.2)
            .foregroundColor(.green)
            .previewLayout(.fixed(width: 200, height: 5))
    }
}
