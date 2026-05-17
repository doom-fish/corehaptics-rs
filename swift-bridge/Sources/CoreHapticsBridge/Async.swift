import CoreHaptics
import Dispatch
import Foundation

// Async thunk for CHHapticEngine.start()
// Callback signature: (result: *const c_void, error: *const i8, ctx: *mut c_void) -> Void
// For this operation we don't return a result, so result is always NULL.
@_cdecl("chrs_engine_start_async")
public func chrs_engine_start_async(
    _ rawEngine: UnsafeMutableRawPointer?,
    cb: @escaping @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer?) -> Void,
    ctx: UnsafeMutableRawPointer?
) {
    let engineBox = chrsEngineBox(rawEngine)
    Task {
        do {
            try await engineBox.engine.start()
            cb(nil, nil, ctx)
        } catch {
            let errorDesc = (error as NSError).localizedDescription
            errorDesc.withCString { errorCStr in
                cb(nil, errorCStr, ctx)
            }
        }
    }
}

// Async thunk for CHHapticEngine.stop()
// Callback signature: (result: *const c_void, error: *const i8, ctx: *mut c_void) -> Void
// For this operation we don't return a result, so result is always NULL.
@_cdecl("chrs_engine_stop_async")
public func chrs_engine_stop_async(
    _ rawEngine: UnsafeMutableRawPointer?,
    cb: @escaping @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer?) -> Void,
    ctx: UnsafeMutableRawPointer?
) {
    let engineBox = chrsEngineBox(rawEngine)
    Task {
        do {
            try await engineBox.engine.stop()
            cb(nil, nil, ctx)
        } catch {
            let errorDesc = (error as NSError).localizedDescription
            errorDesc.withCString { errorCStr in
                cb(nil, errorCStr, ctx)
            }
        }
    }
}

// Async thunk for CHHapticEngine.notifyWhenPlayersFinished()
// Callback signature: (result: *const c_void, error: *const i8, ctx: *mut c_void) -> Void
// The result is nil for success (we don't return the action to Rust in the async version).
@_cdecl("chrs_engine_notify_when_players_finished_async")
public func chrs_engine_notify_when_players_finished_async(
    _ rawEngine: UnsafeMutableRawPointer?,
    cb: @escaping @convention(c) (UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer?) -> Void,
    ctx: UnsafeMutableRawPointer?
) {
    let engineBox = chrsEngineBox(rawEngine)
    engineBox.engine.notifyWhenPlayersFinished { [engineBox] error in
        _ = engineBox
        if let error {
            let errorDesc = (error as NSError).localizedDescription
            errorDesc.withCString { errorCStr in
                cb(nil, errorCStr, ctx)
            }
        } else {
            cb(nil, nil, ctx)
        }
        return .leaveEngineRunning
    }
}

