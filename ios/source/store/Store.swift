import Foundation

class Store: ObservableObject {
    let reducer: Reducer
    var middlewares: [Middleware]
    @Published private(set) var state: AppState

    init(reducer: @escaping Reducer, state: AppState, middlewares: [Middleware]) {
        self.reducer = reducer
        self.middlewares = middlewares
        self.state = state
        
        for var middleware in self.middlewares {
            middleware.dispatch = dispatch
        }
    }

    func dispatch(_ action: Action) {
        DispatchQueue.main.async {
            self.state = self.reducer(self.state, action)
        }

        for middleware in middlewares {
            middleware.execute(state: self.state, action: action)
        }
    }
}
