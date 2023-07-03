import Foundation

protocol CoreConnectionDelegate: AnyObject {
    func coreConnectionDidConnect()
    func coreConnectionDidDisconnect()
    func coreConnectionDidReceiveData(data: Data)
    func coreConnectionDidReceiveString(string: String)
}

class CoreConnection: NSObject, URLSessionWebSocketDelegate {
    private var task: URLSessionWebSocketTask?
    private(set) var connected = false
    weak var delegate: CoreConnectionDelegate?

    func connect(_ ipAddress: String) {
        let url = URL(string: "ws://\(ipAddress):8999")!
        let request = URLRequest(url: url)
        let session = URLSession(
            configuration: .default, delegate: self, delegateQueue: OperationQueue())
        task = session.webSocketTask(with: request)
        task?.maximumMessageSize = 10 * 1024 * 1024
        task?.resume()
    }

    private func disconnect() {
        self.task?.cancel()

        print("Disconnect from core")

        connected = false

        DispatchQueue.main.async {
            self.delegate?.coreConnectionDidDisconnect()
        }
    }

    func send(_ data: Data) {
        if connected {
            task?.send(
                .data(data),
                completionHandler: { error in
                    if let error = error {
                        print("Error sending to core: \(error)")
                        self.disconnect()
                    }
                })
        }

    }

    private func receive() {
        task?.receive { [weak self] result in
            switch result {
            case .success(let message):
                DispatchQueue.main.async {
                    switch message {
                    case .data(let data):
                        self?.delegate?.coreConnectionDidReceiveData(data: data)
                    case .string(let string):
                        self?.delegate?.coreConnectionDidReceiveString(string: string)
                    @unknown default:
                        print("Received unknown message")
                    }
                }

            case .failure(let error):
                print("Error receiving data: \(error)")
                self?.disconnect()
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

        connected = true

        DispatchQueue.main.async {
            self.delegate?.coreConnectionDidConnect()
        }

        receive()
    }

    func urlSession(
        _ session: URLSession, webSocketTask: URLSessionWebSocketTask,
        didCloseWith closeCode: URLSessionWebSocketTask.CloseCode, reason: Data?
    ) {
        self.disconnect()
    }
}
