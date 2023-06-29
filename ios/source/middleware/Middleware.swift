//
//  Middleware.swift
//  Bloop
//
//  Created by Joe Noel on 29/06/2023.
//

import Foundation

protocol Middleware {
    mutating func execute(state: AppState, action: Action, dispatch: @escaping (Action) -> Void)
}
