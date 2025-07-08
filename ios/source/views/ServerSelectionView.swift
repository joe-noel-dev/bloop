import SwiftUI

struct ServerSelectionView: View {
    var servers: [Server]
    var scanning: Bool
    var onServerSelected: (Server) -> Void
    var onLocalSelected: () -> Void
    var onCancel: (() -> Void)? = nil
    var onRestartScan: (() -> Void)? = nil

    var body: some View {
        NavigationView {
            VStack(spacing: Layout.units(4)) {

                if onCancel != nil {
                    Text("Select Server")
                        .font(.title2)
                        .fontWeight(.semibold)
                        .padding(.top, Layout.units(2))
                }
                else {
                    Spacer()
                }

                VStack(spacing: Layout.units(2)) {
                    Button(
                        action: {
                            onLocalSelected()
                        },
                        label: {
                            HStack {
                                Image(systemName: "desktopcomputer")
                                    .foregroundColor(.primary)
                                Text(onCancel != nil ? "Start Local" : "Start")
                                    .font(.title2)
                                    .fontWeight(.semibold)
                            }
                            .frame(maxWidth: .infinity)
                        }
                    )
                    .buttonStyle(.borderedProminent)
                    .controlSize(.large)
                    .padding(.horizontal, Layout.units(4))

                    if onCancel != nil {
                        Text("Run Bloop locally on this device")
                            .font(.subheadline)
                            .foregroundColor(.secondary)
                            .padding(.horizontal, Layout.units(4))
                    }
                }

                if servers.isEmpty && scanning {
                    VStack(spacing: Layout.units(2)) {
                        ProgressView()
                            .progressViewStyle(CircularProgressViewStyle())
                            .scaleEffect(1.5)

                        Text("Scanning for servers...")
                            .font(.subheadline)
                            .foregroundColor(.secondary)
                    }
                    .padding(.top, Layout.units(4))
                }

                if servers.isEmpty && !scanning {
                    VStack(spacing: Layout.units(2)) {
                        Image(systemName: "wifi.slash")
                            .font(.system(size: 48))
                            .foregroundColor(.secondary)

                        Text("No servers found")
                            .font(.headline)

                        Text(
                            "Make sure your device is connected to the same network as the Bloop server."
                        )
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                        .multilineTextAlignment(.center)
                    }
                    .padding(.horizontal, Layout.units(4))
                    .padding(.top, Layout.units(4))
                }

                if !servers.isEmpty {
                    VStack(alignment: .leading, spacing: Layout.units(2)) {
                        Text("Available Servers")
                            .font(.headline)
                            .padding(.horizontal, Layout.units(2))

                        ForEach(servers, id: \.self) { server in
                            HStack {
                                Text(displayName(server))
                                    .font(.body)
                                    .frame(maxWidth: .infinity, alignment: .leading)

                                Button(
                                    action: {
                                        onServerSelected(server)
                                    },
                                    label: {
                                        Text("Connect")
                                            .fontWeight(.medium)
                                    }
                                )
                                .buttonStyle(.bordered)
                            }
                            .padding()
                            .background(Color(.secondarySystemBackground))
                            .cornerRadius(10)
                            .padding(.horizontal, Layout.units(2))
                        }
                    }
                }

                Spacer()

                if let onRestartScan = onRestartScan {
                    Button(
                        action: {
                            onRestartScan()
                        },
                        label: {
                            Text("Restart scan")
                                .font(.subheadline)
                        }
                    )
                    .buttonStyle(.plain)
                    .foregroundColor(.secondary)
                    .padding(.bottom, Layout.units(2))
                }

                Text(version)
                    .font(.caption2)
                    .foregroundColor(.gray)
                    .padding(.bottom, Layout.units(1))
            }
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                if let onCancel = onCancel {
                    ToolbarItem(placement: .navigationBarLeading) {
                        Button("Cancel") {
                            onCancel()
                        }
                    }
                }
            }
        }
    }

    private var version: String {
        guard
            let versionString = Bundle.main.object(
                forInfoDictionaryKey: "CFBundleShortVersionString"
            ) as? String
        else {
            print("Version string not found")
            return "0.0.0"
        }

        return versionString
    }
}

private func displayName(_ server: Server) -> String {
    switch server {
    case .hostPort(let host, let port):
        return "\(host):\(port)"
    case .service(let name, type: _, domain: _, interface: _):
        return name
    case .unix(let path):
        return path
    case .url(let url):
        return url.absoluteString
    case .opaque(_):
        return "Bloop"
    @unknown default:
        return ""
    }
}

struct ServerSelectionView_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            // Sheet mode (with cancel)
            ServerSelectionView(
                servers: [],
                scanning: true,
                onServerSelected: { _ in },
                onLocalSelected: {},
                onCancel: {}
            )

            // Full screen mode (without cancel, with restart scan)
            ServerSelectionView(
                servers: [
                    .hostPort(host: "192.168.1.100", port: 8080),
                    .service(
                        name: "MacBook Pro",
                        type: "_bloop._tcp.",
                        domain: "local.",
                        interface: nil
                    ),
                ],
                scanning: false,
                onServerSelected: { _ in },
                onLocalSelected: {},
                onRestartScan: {}
            )
        }
    }
}
