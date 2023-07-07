import SwiftUI

struct WaveformView: View {
    let waveform: WaveformData?

    var body: some View {
        ZStack {
            EmptyView()

            if let waveform = waveform {
                Canvas { context, size in
                    context.fill(
                        createWaveformPath(waveform: waveform, size: size),
                        with: .foreground
                    )
                }
                .frame (maxWidth: .infinity, maxHeight: .infinity)
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(.thinMaterial)
        .cornerRadius(Layout.corderRadiusMedium)
    }
}

func chooseWaveformGroupForWidth(
    waveform: WaveformData,
    width: CGFloat,
    algorithm: WaveformAlgorithm
) -> WaveformGroup? {
    return waveform.peaks
        .filter { group in
            group.properties.algorithm == algorithm
        }
        .filter { group in
            group.values.count > Int(ceil(width))
        }.sorted { a, b in
            a.properties.length > b.properties.length
        }.first
}

func createWaveformPath(waveform: WaveformData, size: CGSize) -> Path {
    let minWaveform = chooseWaveformGroupForWidth(
        waveform: waveform,
        width: size.width,
        algorithm: .min
    )
    
    let maxWaveform = chooseWaveformGroupForWidth(
        waveform: waveform,
        width: size.width,
        algorithm: .max
    )
    
    guard
        let minWaveform = minWaveform,
            let maxWaveform = maxWaveform,
            minWaveform.values.count == maxWaveform.values.count
    else {
        return Path()
    }
    
    var minPath = Path()
    var maxPath = Path()

    minPath.move(to: .init(x: 0, y: 0.5))
    maxPath.move(to: .init(x: 0, y: 0.5))

    zip(minWaveform.values, maxWaveform.values)
        .enumerated()
        .forEach { (offset, values) in
            let (minValue, maxValue) = values
            let x = Double(offset) / Double(minWaveform.values.count)

            let minY = max(0.5, (1.0 - Double(minValue)) / 2.0)
            let maxY = min(0.5, (1.0 - Double(maxValue)) / 2.0)

            minPath.addLine(to: .init(x: x, y: minY))
            maxPath.addLine(to: .init(x: x, y: maxY))
        }

    minPath.addLine(to: .init(x: 1, y: 0.5))
    maxPath.addLine(to: .init(x: 1, y: 0.5))
    
    minPath.closeSubpath()
    maxPath.closeSubpath()
    
    let transform = CGAffineTransform.init(scaleX: size.width, y: size.height)
        
    var path = Path()
    path.addPath(minPath, transform: transform)
    path.addPath(maxPath, transform: transform)
    return path
}

struct WaveformView_Previews: PreviewProvider {
    static var previews: some View {
        WaveformView(waveform: nil)
            .previewLayout(.fixed(width: 800, height: 300))
            .padding()
    }
}
