import SwiftUI

struct DisconnectedView: View {

    var dispatch: Dispatch

    var body: some View {
        VStack(spacing: Layout.units(2)) {

            Button(
                action: {
                    dispatch(.browse)
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
