import SwiftUI

struct LoginView: View {
    @State private var username: String = ""
    @State private var password: String = ""
    @State private var isLoggingIn: Bool = false
    var state: AppState
    var dispatch: Dispatch
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        ScrollView {
            VStack(spacing: 0) {
                // Header Section
                VStack(spacing: Layout.units(1.5)) {
                    Image(systemName: "waveform.circle.fill")
                        .font(.system(size: Layout.iconLarge))
                        .foregroundStyle(Colours.primaryGradient)
                        .padding(.bottom, Layout.units(1))
                    
                    Text("Bloop")
                        .font(.largeTitle)
                        .fontWeight(.bold)
                        .foregroundColor(.primary)
                    
                    Text("Sign in to continue")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }
                .padding(.top, Layout.units(7.5))
                .padding(.bottom, Layout.units(6))
                
                // Form Section
                VStack(spacing: Layout.units(2.5)) {
                    VStack(alignment: .leading, spacing: Layout.units(1)) {
                        Text("Email")
                            .font(.subheadline)
                            .fontWeight(.medium)
                            .foregroundColor(.secondary)
                        
                        TextField("Enter your email", text: $username)
                            .textFieldStyle(.plain)
                            .autocapitalization(.none)
                            .autocorrectionDisabled()
                            .keyboardType(.emailAddress)
                            .textContentType(.username)
                            .padding(Layout.units(2))
                            .background(Color(.systemGray6))
                            .cornerRadius(Layout.cornerRadiusXLarge)
                            .overlay(
                                RoundedRectangle(cornerRadius: Layout.cornerRadiusXLarge)
                                    .stroke(Color(.systemGray4), lineWidth: Layout.borderThin)
                            )
                            .disabled(isLoggingIn)
                    }
                    
                    VStack(alignment: .leading, spacing: Layout.units(1)) {
                        Text("Password")
                            .font(.subheadline)
                            .fontWeight(.medium)
                            .foregroundColor(.secondary)
                        
                        SecureField("Enter your password", text: $password)
                            .textFieldStyle(.plain)
                            .textContentType(.password)
                            .padding(Layout.units(2))
                            .background(Color(.systemGray6))
                            .cornerRadius(Layout.cornerRadiusXLarge)
                            .overlay(
                                RoundedRectangle(cornerRadius: Layout.cornerRadiusXLarge)
                                    .stroke(Color(.systemGray4), lineWidth: Layout.borderThin)
                            )
                            .disabled(isLoggingIn)
                    }
                    
                    Button(action: {
                        isLoggingIn = true
                        let action = logInAction(email: username, password: password)
                        dispatch(action)
                    }) {
                        HStack(spacing: Layout.units(1.5)) {
                            if isLoggingIn {
                                ProgressView()
                                    .tint(.white)
                            }
                            Text(isLoggingIn ? "Signing in..." : "Sign In")
                                .fontWeight(.semibold)
                        }
                        .frame(maxWidth: .infinity)
                        .padding(Layout.units(2))
                        .background(isLoggingIn ? Colours.disabledGradient : Colours.buttonGradient)
                        .foregroundColor(.white)
                        .cornerRadius(Layout.cornerRadiusXLarge)
                    }
                    .disabled(isLoggingIn || username.isEmpty || password.isEmpty)
                    .opacity((username.isEmpty || password.isEmpty) && !isLoggingIn ? 0.6 : 1.0)
                    .padding(.top, Layout.units(1))
                }
                .padding(.horizontal, Layout.units(4))
                
                Spacer()
            }
        }
        .background(Color(.systemBackground))
        .onChange(of: state.user) { _, newUser in
            if newUser != nil {
                dismiss()
            }
        }
    }
}
