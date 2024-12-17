import Foundation
import Network

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
    private(set) var state: ConnectionState = .disconnected
    weak var delegate: CoreConnectionDelegate?
    private var connection: NWConnection?
    private var queue = DispatchQueue(label: "com.core.connection")

    func connect(_ endpoint: NWEndpoint) {

        let options = NWProtocolWebSocket.Options()
        options.autoReplyPing = true
        options.maximumMessageSize = 20 * 1024 * 1024
        let params = NWParameters(tls: nil, tcp: .init())
        params.defaultProtocolStack.applicationProtocols.insert(options, at: 0)

        connection = NWConnection(to: endpoint, using: params)

        connection?.stateUpdateHandler = { state in
            print("Connection state: \(state)")
            switch state {
            case .ready:
                print("Connected to Core")

                DispatchQueue.main.async {
                    self.delegate?.coreConnectionDidConnect()
                }

                self.state = .connected
                self.receive()
                break
            case .failed(_):
                self.disconnect()
                break
            case .setup:
                break
            case .waiting(_):
                break
            case .preparing:
                break
            case .cancelled:
                self.disconnect()
                break
            @unknown default:
                break
            }
        }

        connection?.start(queue: queue)
        state = .connecting

    }

    func send(_ data: Data) {
        let metadata = NWProtocolWebSocket.Metadata(opcode: .binary)
        let context = NWConnection.ContentContext(
            identifier: "CoreConnection content context",
            metadata: [metadata]
        )

        connection?.send(
            content: data,
            contentContext: context,
            completion: .contentProcessed { error in
                if let error = error {
                    print("Send failed: \(error.localizedDescription)")
                    self.disconnect()
                }
            }
        )
    }

    func receive() {
        connection?.receiveMessage(completion: { data, context, isComplete, error in

            if let error = error {
                print("Receive error: \(error.localizedDescription)")
                self.disconnect()
                return
            }
            
            if let data = data {
                DispatchQueue.main.async {
                    self.delegate?.coreConnectionDidReceiveData(data: data)
                }
            }

            self.receive()
        })
    }

    private func disconnect() {
        guard self.state != .disconnected else {
            return
        }

        self.connection?.cancel()

        print("Disconnected from core")

        self.state = .disconnected

        DispatchQueue.main.async {
            self.delegate?.coreConnectionDidDisconnect()
        }
    }
}
