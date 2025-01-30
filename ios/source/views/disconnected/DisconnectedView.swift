import SwiftUI

struct DisconnectedView: View {
    var servers: [Server]
    var scanning: Bool
    var dispatch: Dispatch

    var body: some View {
        VStack(spacing: Layout.units(2)) {

            Button(
                action: {
                    dispatch(.restartScan)
                },
                label: {
                    Text("Restart scan")
                        .fontWeight(.bold)
                }
            ).buttonStyle(.bordered)

            Spacer()

            if servers.isEmpty && scanning {
                ProgressView()
                    .progressViewStyle(CircularProgressViewStyle())
                    .scaleEffect(2.0)
            }

            ForEach(self.servers, id: \.self) { server in
                Button(
                    action: {
                        dispatch(.connect(server))
                    },
                    label: {
                        Text(displayName(server))
                            .font(.headline)
                            .frame(maxWidth: .infinity)
                    }
                ).buttonStyle(.borderedProminent)
            }

            Spacer()

            Text(version)
                .font(.caption)
        }
        .padding(Layout.units(2))
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
        DisconnectedView(
            servers: [],
            scanning: false,
            dispatch: loggingDispatch
        )
    }
}
