import SwiftUI

struct DisconnectedView: View {
    var dispatch: (Action) -> Void

    var body: some View {
        VStack(spacing: 16) {
            Text("Disconnected")
                .font(.largeTitle)

            Button(
                action: {
                    dispatch(.connect)
                },
                label: {
                    Text("Connect")
                        .font(.title)
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
