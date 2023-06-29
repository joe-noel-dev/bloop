import Foundation

class CoreConnection: NSObject, URLSessionWebSocketDelegate {
    private var task: URLSessionWebSocketTask?
    private var connected = false

    override init() {
        super.init()
        self.connect()
    }

    private func connect() {
        let url = URL(string: "ws://localhost:8999")!
        let request = URLRequest(url: url)
        let session = URLSession(
            configuration: .default, delegate: self, delegateQueue: OperationQueue())
        let task = session.webSocketTask(with: request)
        task.resume()
    }

    private func send(_ data: Data) {
        task?.send(
            .data(data),
            completionHandler: { error in
                if let error = error {
                    print("Error sending to core: \(error)")
                }
            })
    }

    private func receive() {
        task?.receive { [weak self] result in
            switch result {
            case .success(let message):
                switch message {
                case .data(let data):
                    print("Received data: \(data.count) bytes")
                case .string(let string):
                    print("Received string: \(string)")
                @unknown default:
                    print("Received unknown message")
                }

            case .failure(let error):
                print("Error receiving data: \(error)")

            }

            guard let connected = self?.connected else {
                return
            }

            if connected {
                self?.receive()
            }
        }

    }

    func urlSession(
        _ session: URLSession, webSocketTask: URLSessionWebSocketTask,
        didOpenWithProtocol protocol: String?
    ) {
        print("Connected to core")
        self.connected = true
        self.receive()
    }

    func urlSession(
        _ session: URLSession, webSocketTask: URLSessionWebSocketTask,
        didCloseWith closeCode: URLSessionWebSocketTask.CloseCode, reason: Data?
    ) {
        self.connected = false
        print("Disconnect from core")

    }
}
