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

            ForEach(self.servers) { server in
                Button(
                    action: {
                        dispatch(.connect(server))
                    },
                    label: {
                        Text(displayName(server.hostname))
                            .font(.headline)
                            .frame(maxWidth: .infinity)
                    }
                ).buttonStyle(.borderedProminent)
                    
            }
        }
        .padding(Layout.units(2))
        .onAppear {
            dispatch(.browse)
        }

    }
}

private func displayName(_ hostname: String) -> String {
    if let index = hostname.firstIndex(of: ".") {
        return String(hostname[..<index])
    }
    return hostname
}

struct DisconnectedView_Previews: PreviewProvider {
    static var previews: some View {
        DisconnectedView(
            servers: [
                .init(hostname: "Hostname 1", port: 1234),
                .init(hostname: "Hostname 2", port: 5678),
            ],
            dispatch: loggingDispatch
        )
    }
}
