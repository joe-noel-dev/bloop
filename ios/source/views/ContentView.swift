import SwiftUI

struct ContentView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        if store.state.connected != nil {
            ProjectView(
                state: store.state
            ) {
                action in
                store.dispatch(action)
            }
        }
        else {
            ServerSelectionView(
                servers: store.state.servers,
                scanning: store.state.scanning,
                onServerSelected: { server in
                    store.dispatch(.connect(server))
                },
                onLocalSelected: {
                    store.dispatch(.connectLocal)
                },
                onRestartScan: {
                    store.dispatch(.restartScan)
                }
            )
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
