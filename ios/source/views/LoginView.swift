import SwiftUI

struct LoginView: View {
    @State private var username: String = ""
    @State private var password: String = ""
    @State private var isLoggingIn: Bool = false
    var state: AppState
    var dispatch: Dispatch
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        VStack(spacing: 20) {
            TextField("Username", text: $username)
                .textFieldStyle(RoundedBorderTextFieldStyle())
                .autocapitalization(.none)
                .padding(.horizontal)
                .disabled(isLoggingIn)
            
            SecureField("Password", text: $password)
                .textFieldStyle(RoundedBorderTextFieldStyle())
                .padding(.horizontal)
                .disabled(isLoggingIn)
            
            Button(action: {
                isLoggingIn = true
                let action = logInAction(email: username, password: password)
                dispatch(action)
            }) {
                HStack {
                    if isLoggingIn {
                        ProgressView()
                            .scaleEffect(0.8)
                            .tint(.white)
                        Text("Logging in...")
                    } else {
                        Text("Submit")
                    }
                }
                .frame(maxWidth: .infinity)
                .padding()
                .background(isLoggingIn ? Color.gray : Color.blue)
                .foregroundColor(.white)
                .cornerRadius(8)
            }
            .disabled(isLoggingIn)
            .padding(.horizontal)
        }
        .padding()
        .onChange(of: state.user) { _, newUser in
            if newUser != nil {
                dismiss()
            }
        }
    }
}
