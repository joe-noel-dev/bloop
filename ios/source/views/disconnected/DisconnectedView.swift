import SwiftUI

struct DisconnectedView: View {
    var servers: [Server]
    var dispatch: Dispatch

    var body: some View {
        VStack(spacing: Layout.units(2)) {

            if servers.isEmpty {
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
        }
        .padding(Layout.units(2))
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
            dispatch: loggingDispatch
        )
    }
}
