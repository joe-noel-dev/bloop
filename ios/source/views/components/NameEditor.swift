import SwiftUI

struct NameEditor: View {
    var prompt: String
    @Binding var value: String
    @FocusState private var textFieldIsFocused: Bool

    var body: some View {
        Form {
            Section(prompt) {
                TextField(prompt, text: $value)
                    #if os(iOS)
                        .textInputAutocapitalization(.words)
                    #endif
                    .disableAutocorrection(true)
                    .focused($textFieldIsFocused)
            }
        }
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
