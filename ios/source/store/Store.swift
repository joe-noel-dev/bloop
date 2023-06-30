import Foundation

class Store: ObservableObject {
    let reducer: Reducer
    let middlewares: [Middleware]
    @Published private(set) var state: AppState

    init(reducer: @escaping Reducer, state: AppState, middlewares: [Middleware]) {
        self.reducer = reducer
        self.middlewares = middlewares
        self.state = state
    }

    func dispatch(_ action: Action) {
        DispatchQueue.main.async {
            self.state = self.reducer(self.state, action)
        }

        for var middleware in middlewares {
            middleware.execute(state: self.state, action: action) { action in
                self.dispatch(action)
            }
        }
    }
}
