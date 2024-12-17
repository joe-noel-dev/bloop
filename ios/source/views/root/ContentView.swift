import SwiftUI

struct ContentView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        if store.state.connected {
            ProjectView(
                state: store.state
            ) {
                action in
                store.dispatch(action)
            }
        }
        else {
            DisconnectedView(servers: store.state.servers) { action in
                store.dispatch(action)
            }
        }

    }
}

struct ContentView_Previews: PreviewProvider {
    static let store = Store(reducer: rootReducer, state: AppState(), middlewares: [])

    static var previews: some View {
        Group {
            ContentView()
                .environmentObject(store)
        }

    }
}
