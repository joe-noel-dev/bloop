import SwiftUI

struct DisconnectedView: View {

    @AppStorage("ip-address") private var ipAddress = "localhost"
    var dispatch: Dispatch

    var body: some View {
        VStack(spacing: Layout.units(2)) {

            TextField("IP Address", text: $ipAddress)
                .multilineTextAlignment(.center)
                .font(.title)
                .frame(minHeight: 64)
                .textFieldStyle(.roundedBorder)
                #if os(iOS)
                    .keyboardType(.decimalPad)
                #endif

            Button(
                action: {
                    dispatch(.connect(ipAddress))
                },
                label: {
                    Text("Connect")
                        .font(.title)
                }
            ).buttonStyle(.borderedProminent)
        }
        .padding(Layout.units(2))

    }
}

struct DisconnectedView_Previews: PreviewProvider {
    static var previews: some View {
        DisconnectedView(dispatch: loggingDispatch)
    }
}
