import SwiftUI

struct DisconnectedView: View {
    @State private var ipAddress = "localhost"
    var dispatch: Dispatch

    var body: some View {
        VStack(spacing: 16) {
            Text("Disconnected")
                .font(.largeTitle)

            TextField("IP Address", text: $ipAddress)
                .multilineTextAlignment(.center)
                .border(.black)

            Button(
                action: {
                    dispatch(.connect(ipAddress))
                },
                label: {
                    Text("Connect")
                        .font(.title)
                }
            )
        }
        .padding()

    }
}

struct DisconnectedView_Previews: PreviewProvider {
    static var previews: some View {
        DisconnectedView(dispatch: loggingDispatch)
    }
}
