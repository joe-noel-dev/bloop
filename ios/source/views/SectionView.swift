import SwiftUI

struct SectionView: View {
    var section: Section

    var body: some View {
        Text(section.name)
            .padding(16)
            .background(.green)
    }
}

struct SectionView_Previews: PreviewProvider {

    static var section: Section = {
        var section = demoSection(0)
        section.name = "Chorus"
        return section
    }()

    static var previews: some View {
        SectionView(section: section)
            .previewLayout(.sizeThatFits)
            .padding()
    }
}
