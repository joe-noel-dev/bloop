import SwiftUI

struct DisconnectedView: View {
    var dispatch: (Action) -> Void

    var body: some View {
        VStack(spacing: 16) {
            Text("Disconnected")

            Button(
                "Connect",
                action: {
                    dispatch(.connect)
                })
        }

    }
}

struct DisconnectedView_Previews: PreviewProvider {
    static var previews: some View {
        DisconnectedView { action in
            print("Dispatch -> \(action)")
        }
    }
}
