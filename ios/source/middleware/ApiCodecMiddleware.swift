import Foundation

class ApiCodecMiddleware: Middleware {
    var dispatch: Dispatch?

    var queue = DispatchQueue(label: "ApiCodecMiddleware")

    func execute(state: AppState, action: Action) {
        if case .receivedRawResponse(let data) = action {
            onReceivedRawResponse(data)
        }

        if case .sendRequest(let request) = action {
            onSendRequest(request)
        }
    }

    private func onReceivedRawResponse(_ data: Data) {
        queue.async { [weak self] in
            do {
                let response = try Bloop_Response(serializedBytes: data)

                DispatchQueue.main.async {
                    self?.dispatch?(.receivedResponse(response))
                }
            }
            catch {
                print("Error decoding response from core: \(error)")
            }
        }
    }

    private func onSendRequest(_ request: Bloop_Request) {
        queue.async { [weak self] in
            do {

                let encodedRequest = try request.serializedData()

                DispatchQueue.main.async {
                    self?.dispatch?(.sendRawRequest(encodedRequest))
                }
            }
            catch {
                print("Error encoding request: \(error)")
            }
        }
    }

}
