import SwiftUI

struct NameEditor: View {
    @Binding var value: String
    @FocusState private var textFieldIsFocused: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: Layout.units(2)) {
            Text("Name")

            TextField("New Name", text: $value)
                .textFieldStyle(.roundedBorder)
                .textInputAutocapitalization(.words)
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
        NameEditor(value: $name)
            .previewLayout(.sizeThatFits)
    }
}
