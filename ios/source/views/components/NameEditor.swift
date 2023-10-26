import SwiftUI

struct NameEditor: View {
    var prompt: String
    @Binding var value: String
    @FocusState private var textFieldIsFocused: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: Layout.units(2)) {
            Text(prompt)

            TextField(prompt, text: $value)
                .textFieldStyle(.roundedBorder)
                #if os(iOS)
                    .textInputAutocapitalization(.words)
                #endif
                .disableAutocorrection(true)
                .focused($textFieldIsFocused)
        }
        .font(.title2)
        .padding(Layout.units(2))
        .frame(minWidth: 400)
        .background(.regularMaterial)
        .onAppear {
            textFieldIsFocused = true
        }
    }
}

struct NameEditor_Previews: PreviewProvider {
    @State
    static var name: String = "Hello"

    static var previews: some View {
        NameEditor(prompt: "Name", value: $name)
            .previewLayout(.sizeThatFits)
    }
}
