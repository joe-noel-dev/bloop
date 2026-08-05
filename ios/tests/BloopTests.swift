//
//  BloopTests.swift
//  BloopTests
//
//  Created by Joe Noel on 29/06/2023.
//

import XCTest

@testable import Bloop

final class BloopTests: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testExample() throws {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct results.
        // Any test you write for XCTest can be annotated as throws and async.
        // Mark your test throws to produce an unexpected failure when your test encounters an uncaught error.
        // Mark your test async to allow awaiting for asynchronous code to complete. Check the results with assertions afterwards.
    }

    func testEmptyProjectsSnapshotClearsBothProjectLists() throws {
        var dispatched: [Action] = []
        let middleware = ResponseMiddleware()
        middleware.dispatch = { dispatched.append($0) }

        let response = Bloop_Response.with {
            $0.projectsSnapshot = Bloop_ProjectsSnapshot.with {
                $0.cloudAvailable = false
            }
        }

        middleware.execute(state: AppState(), action: .receivedResponse(response))

        XCTAssertTrue(dispatched.contains(where: {
            if case .setProjectsSnapshot(let projects, let cloudProjects, let cloudAvailable) = $0 {
                return projects.isEmpty && cloudProjects.isEmpty && !cloudAvailable
            }
            return false
        }))
    }

    func testPerformanceExample() throws {
        // This is an example of a performance test case.
        measure {
            // Put the code you want to measure the time of here.
        }
    }

}
