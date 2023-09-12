import Foundation

protocol CoreConnectionDelegate: AnyObject {
    func coreConnectionDidConnect()
    func coreConnectionDidDisconnect()
    func coreConnectionDidReceiveData(data: Data)
    func coreConnectionDidReceiveString(string: String)
}

enum ConnectionState {
    case disconnected
    case connecting
    case connected
}

class CoreConnection: NSObject, URLSessionWebSocketDelegate, URLSessionTaskDelegate {
    private var task: URLSessionWebSocketTask?
    private(set) var state: ConnectionState = .disconnected
    weak var delegate: CoreConnectionDelegate?

    func connect(hostname: String, port: Int) {
        let url = URL(string: "ws://\(hostname):\(port)")!
        let session = URLSession(
            configuration: .default,
            delegate: self,
            delegateQueue: OperationQueue()
        )

        task = session.webSocketTask(with: url)
        task?.maximumMessageSize = 20 * 1024 * 1024
        task?.resume()
        self.state = .connecting
    }

    private func disconnect() {
        guard self.state != .disconnected else {
            return
        }

        self.task?.cancel()

        print("Disconnected from core")

        self.state = .disconnected

        DispatchQueue.main.async {
            self.delegate?.coreConnectionDidDisconnect()
        }
    }

    func send(_ data: Data) {

        guard self.state == .connected else {
            return
        }

        task?.send(
            .data(data),
            completionHandler: { error in
                if let error = error {
                    print("Error sending to core: \(error)")
                    self.disconnect()
                }
            }
        )
    }

    private func receive() {
        guard let task = self.task else {
            return
        }

        task.receive { [weak self] result in
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
                print("Error receiving data: \(error.localizedDescription)")
                self?.disconnect()
                return
            }

            guard let self = self else {
                return
            }

            if self.state == .connected {
                self.receive()
            }
        }
    }

    func urlSession(
        _ session: URLSession,
        webSocketTask: URLSessionWebSocketTask,
        didOpenWithProtocol protocol: String?
    ) {
        print("Connected to core")

        self.state = .connected

        DispatchQueue.main.async {
            self.delegate?.coreConnectionDidConnect()
        }

        receive()
    }

    func urlSession(
        _ session: URLSession,
        webSocketTask: URLSessionWebSocketTask,
        didCloseWith closeCode: URLSessionWebSocketTask.CloseCode,
        reason: Data?
    ) {
        self.disconnect()
    }

    func urlSession(_ session: URLSession, task: URLSessionTask, didCompleteWithError error: Error?)
    {
        self.disconnect()
    }
}
