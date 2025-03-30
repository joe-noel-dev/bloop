import SwiftUI

struct DisconnectedView: View {
    var servers: [Server]
    var scanning: Bool
    var dispatch: Dispatch

    var body: some View {
        VStack(spacing: Layout.units(4)) {

            Spacer()

            VStack(spacing: Layout.units(2)) {

                Button(
                    action: {
                        dispatch(.connectLocal)
                    },
                    label: {
                        Text("Start")
                            .font(.title2)
                            .fontWeight(.semibold)
                            .frame(maxWidth: .infinity)
                    }
                )
                .buttonStyle(.borderedProminent)
                .controlSize(.large)
                .padding(.horizontal, Layout.units(4))
            }

            if servers.isEmpty && scanning {
                ProgressView()
                    .progressViewStyle(CircularProgressViewStyle())
                    .scaleEffect(1.8)
                    .padding(.top, Layout.units(4))
            }

            if !servers.isEmpty {
                VStack(alignment: .leading, spacing: Layout.units(2)) {
                    Text("Available Servers")
                        .font(.headline)
                        .padding(.top, Layout.units(4))
                        .padding(.horizontal, Layout.units(2))

                    ForEach(servers, id: \.self) { server in
                        HStack {
                            Text(displayName(server))
                                .font(.body)
                                .frame(maxWidth: .infinity, alignment: .leading)

                            Button(
                                action: {
                                    dispatch(.connect(server))
                                },
                                label: {
                                    Text("Connect")
                                        .fontWeight(.medium)
                                }
                            ).buttonStyle(.bordered)
                        }
                        .padding()
                        .background(Color(.secondarySystemBackground))
                        .cornerRadius(10)
                        .padding(.horizontal, Layout.units(2))
                    }
                }
            }

            Spacer()

            Button(
                action: {
                    dispatch(.restartScan)
                },
                label: {
                    Text("Restart scan")
                        .font(.subheadline)
                }
            )
            .buttonStyle(.plain)
            .foregroundColor(.secondary)
            .padding(.bottom, Layout.units(2))

            Text(version)
                .font(.caption2)
                .foregroundColor(.gray)
                .padding(.bottom, Layout.units(1))
        }
        .padding()
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

struct DisconnectedView_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            DisconnectedView(
                servers: [],
                scanning: true,
                dispatch: loggingDispatch
            )

            DisconnectedView(
                servers: [
                    .hostPort(host: "192.168.1.1", port: 8080),
                    .service(
                        name: "Test Service",
                        type: "_http._tcp.",
                        domain: "local.",
                        interface: nil
                    ),
                ],
                scanning: false,
                dispatch: loggingDispatch
            )
        }
    }
}
