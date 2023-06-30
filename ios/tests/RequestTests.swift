import XCTest

@testable import Bloop

final class RequestTests: XCTestCase {
    func testDecodeGetAllRequest() throws {
        let json = """
            {
                "method": "get",
                "payload": {
                    "entity": "all"
                }
            }
            """

        let request = try JSONDecoder().decode(Request.self, from: Data(json.utf8))
        let expectedRequest = Request.get(EntityId(entity: .all))
        XCTAssertEqual(request, expectedRequest)
    }

    func testEncodeGetAllRequest() throws {
        let request = Request.get(EntityId(entity: .all))
        let encoded = try JSONEncoder().encode(request)
        let jsonOutput = String(decoding: encoded, as: UTF8.self)
        let expectedOutput = "{\"payload\":{\"entity\":\"all\"},\"method\":\"get\"}"
        XCTAssertEqual(jsonOutput, expectedOutput)
    }
}
