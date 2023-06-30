import SwiftUI

struct ContentView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        if store.state.connected {
            ProjectsView()
        } else {
            DisconnectedView()
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var store = Store(reducer: rootReducer, state: AppState(), middlewares: [])
    static var previews: some View {
        ContentView().environmentObject(store)
    }
}
