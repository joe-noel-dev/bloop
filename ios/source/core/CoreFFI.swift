import Foundation

// MARK: - Bridging C types

enum BloopErrorCode: UInt32 {
    case success = 0
    case invalidRequest
    case errorPostingRequest
}

// Forward declaration of opaque C struct
typealias BloopContextRef = OpaquePointer

typealias BloopResponseCallbackC = @convention(c) (
    UnsafeMutableRawPointer?, UnsafePointer<UInt8>?, UInt
) -> Void

@_silgen_name("bloop_init")
private func bloopInit(
    _ responseCallback: BloopResponseCallbackC?,
    _ responseCallbackContext: UnsafeMutableRawPointer?
) -> BloopContextRef?

@_silgen_name("bloop_add_request")
private func bloopAddRequest(
    _ context: BloopContextRef?,
    _ request: UnsafePointer<UInt8>?,
    _ size: UInt
) -> BloopErrorCode

@_silgen_name("bloop_shutdown")
private func bloopShutdown(_ ctx: BloopContextRef?)

// MARK: - Swift wrapper

class CoreFFI {
    private let context: BloopContextRef
    private let callbackStorage: CallbackStorage

    typealias ResponseHandler = (_ response: Data) -> Void

    private class CallbackStorage {
        let handler: ResponseHandler
        init(handler: @escaping ResponseHandler) {
            self.handler = handler
        }
    }

    init?(responseHandler: @escaping ResponseHandler) {
        self.callbackStorage = CallbackStorage(handler: responseHandler)

        let contextPtr = Unmanaged.passUnretained(callbackStorage).toOpaque()

        print("Initializing Bloop via FFI")

        guard
            let ctx = bloopInit(
                { context, data, size in
                    guard let context = context,
                        let data = data
                    else { return }

                    let storage = Unmanaged<CallbackStorage>.fromOpaque(context)
                        .takeUnretainedValue()
                    let responseData = Data(bytes: data, count: Int(size))

                    storage.handler(responseData)

                },
                contextPtr
            )
        else {
            return nil
        }

        self.context = ctx

        print("Bloop initialized via FFI")
    }

    func addRequest(_ request: Data) throws {
        let response = request.withUnsafeBytes { bytes in
            guard let baseAddress = bytes.baseAddress?.assumingMemoryBound(to: UInt8.self) else {
                return BloopErrorCode.invalidRequest
            }
            return bloopAddRequest(context, baseAddress, UInt(request.count))
        }

        if response != .success {
            // FIXME: Handle error
            print("Error adding request to Core: \(response)")
        }
    }

    deinit {
        bloopShutdown(context)
    }
}
