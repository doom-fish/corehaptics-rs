import CoreHaptics
import Dispatch
import Foundation

private struct BridgeAudioResourceOptions: Decodable {
    let useVolumeEnvelope: Bool?
    let loopEnabled: Bool?
}

final class EngineStoppedHandlerBox {
    let callback: CHRSEngineStoppedHandler
    let context: UnsafeMutableRawPointer?
    let dropContext: CHRSContextDrop?

    init(
        callback: @escaping CHRSEngineStoppedHandler,
        context: UnsafeMutableRawPointer?,
        dropContext: CHRSContextDrop?
    ) {
        self.callback = callback
        self.context = context
        self.dropContext = dropContext
    }

    func invoke(_ reason: CHHapticEngine.StoppedReason) {
        callback(context, Int32(reason.rawValue))
    }

    deinit {
        dropContext?(context)
    }
}

final class EngineResetHandlerBox {
    let callback: CHRSEngineResetHandler
    let context: UnsafeMutableRawPointer?
    let dropContext: CHRSContextDrop?

    init(
        callback: @escaping CHRSEngineResetHandler,
        context: UnsafeMutableRawPointer?,
        dropContext: CHRSContextDrop?
    ) {
        self.callback = callback
        self.context = context
        self.dropContext = dropContext
    }

    func invoke() {
        callback(context)
    }

    deinit {
        dropContext?(context)
    }
}

final class EngineFinishedHandlerBox {
    let callback: CHRSEngineFinishedHandler
    let context: UnsafeMutableRawPointer?
    let dropContext: CHRSContextDrop?

    init(
        callback: @escaping CHRSEngineFinishedHandler,
        context: UnsafeMutableRawPointer?,
        dropContext: CHRSContextDrop?
    ) {
        self.callback = callback
        self.context = context
        self.dropContext = dropContext
    }

    func invoke(_ error: Error?) -> CHHapticEngine.FinishedAction {
        let errorPointer = error.map { chrsRetain($0 as NSError) }
        switch callback(context, errorPointer) {
        case 1:
            return .stopEngine
        default:
            return .leaveEngineRunning
        }
    }

    deinit {
        dropContext?(context)
    }
}

final class EngineBox: NSObject {
    let engine: CHHapticEngine
    var stoppedHandlerBox: EngineStoppedHandlerBox?
    var resetHandlerBox: EngineResetHandlerBox?
    var finishedHandlerBox: EngineFinishedHandlerBox?

    init(_ engine: CHHapticEngine) {
        self.engine = engine
        super.init()
        engine.stoppedHandler = { _ in }
        engine.resetHandler = {}
    }

    deinit {
        engine.stoppedHandler = { _ in }
        engine.resetHandler = {}
        stoppedHandlerBox = nil
        resetHandlerBox = nil
        finishedHandlerBox = nil
    }
}

func chrsEngineBox(_ raw: UnsafeMutableRawPointer?) -> EngineBox {
    chrsBorrow(raw, as: EngineBox.self)
}

private func chrsAudioResourceOptions(_ raw: UnsafePointer<CChar>?) throws -> [CHHapticAudioResourceKey: Any] {
    guard let raw else { return [:] }
    let options = try chrsDecodeJSON(raw, as: BridgeAudioResourceOptions.self)
    var dictionary: [CHHapticAudioResourceKey: Any] = [:]
    if let useVolumeEnvelope = options.useVolumeEnvelope {
        guard #available(macOS 12.0, *) else {
            throw chrsBridgeNSError(code: 51, message: "UseVolumeEnvelope requires macOS 12.0")
        }
        dictionary[CHHapticAudioResourceKeyUseVolumeEnvelope as NSString] = useVolumeEnvelope
    }
    if let loopEnabled = options.loopEnabled {
        guard #available(macOS 13.0, *) else {
            throw chrsBridgeNSError(code: 52, message: "LoopEnabled requires macOS 13.0")
        }
        dictionary[CHHapticAudioResourceKeyLoopEnabled as NSString] = loopEnabled
    }
    return dictionary
}

@_cdecl("chrs_engine_create")
public func chrs_engine_create(_ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?) -> UnsafeMutableRawPointer? {
    do {
        return chrsRetain(EngineBox(try CHHapticEngine()))
    } catch {
        chrsSetError(errorOut, error)
        return nil
    }
}

@_cdecl("chrs_engine_start")
public func chrs_engine_start(
    _ rawEngine: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        try chrsEngineBox(rawEngine).engine.start()
        return true
    } catch {
        chrsSetError(errorOut, error)
        return false
    }
}

@_cdecl("chrs_engine_stop")
public func chrs_engine_stop(
    _ rawEngine: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    let semaphore = DispatchSemaphore(value: 0)
    var capturedError: Error?
    chrsEngineBox(rawEngine).engine.stop { error in
        capturedError = error
        semaphore.signal()
    }
    semaphore.wait()
    if let capturedError {
        chrsSetError(errorOut, capturedError)
        return false
    }
    return true
}

@_cdecl("chrs_engine_current_time")
public func chrs_engine_current_time(_ rawEngine: UnsafeMutableRawPointer?) -> Double {
    chrsEngineBox(rawEngine).engine.currentTime
}

@_cdecl("chrs_engine_plays_haptics_only")
public func chrs_engine_plays_haptics_only(_ rawEngine: UnsafeMutableRawPointer?) -> Bool {
    chrsEngineBox(rawEngine).engine.playsHapticsOnly
}

@_cdecl("chrs_engine_set_plays_haptics_only")
public func chrs_engine_set_plays_haptics_only(_ rawEngine: UnsafeMutableRawPointer?, _ enabled: Bool) {
    chrsEngineBox(rawEngine).engine.playsHapticsOnly = enabled
}

@_cdecl("chrs_engine_plays_audio_only")
public func chrs_engine_plays_audio_only(_ rawEngine: UnsafeMutableRawPointer?) -> Bool {
    if #available(macOS 13.0, *) {
        return chrsEngineBox(rawEngine).engine.playsAudioOnly
    }
    return false
}

@_cdecl("chrs_engine_set_plays_audio_only")
public func chrs_engine_set_plays_audio_only(_ rawEngine: UnsafeMutableRawPointer?, _ enabled: Bool) {
    guard #available(macOS 13.0, *) else { return }
    chrsEngineBox(rawEngine).engine.playsAudioOnly = enabled
}

@_cdecl("chrs_engine_is_muted_for_audio")
public func chrs_engine_is_muted_for_audio(_ rawEngine: UnsafeMutableRawPointer?) -> Bool {
    chrsEngineBox(rawEngine).engine.isMutedForAudio
}

@_cdecl("chrs_engine_set_muted_for_audio")
public func chrs_engine_set_muted_for_audio(_ rawEngine: UnsafeMutableRawPointer?, _ enabled: Bool) {
    chrsEngineBox(rawEngine).engine.isMutedForAudio = enabled
}

@_cdecl("chrs_engine_is_muted_for_haptics")
public func chrs_engine_is_muted_for_haptics(_ rawEngine: UnsafeMutableRawPointer?) -> Bool {
    chrsEngineBox(rawEngine).engine.isMutedForHaptics
}

@_cdecl("chrs_engine_set_muted_for_haptics")
public func chrs_engine_set_muted_for_haptics(_ rawEngine: UnsafeMutableRawPointer?, _ enabled: Bool) {
    chrsEngineBox(rawEngine).engine.isMutedForHaptics = enabled
}

@_cdecl("chrs_engine_auto_shutdown_enabled")
public func chrs_engine_auto_shutdown_enabled(_ rawEngine: UnsafeMutableRawPointer?) -> Bool {
    chrsEngineBox(rawEngine).engine.isAutoShutdownEnabled
}

@_cdecl("chrs_engine_set_auto_shutdown_enabled")
public func chrs_engine_set_auto_shutdown_enabled(_ rawEngine: UnsafeMutableRawPointer?, _ enabled: Bool) {
    chrsEngineBox(rawEngine).engine.isAutoShutdownEnabled = enabled
}

@_cdecl("chrs_engine_create_player")
public func chrs_engine_create_player(
    _ rawEngine: UnsafeMutableRawPointer?,
    _ rawPattern: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> UnsafeMutableRawPointer? {
    do {
        let player = try chrsEngineBox(rawEngine).engine.makePlayer(with: chrsPatternBox(rawPattern).pattern)
        return chrsRetain(PatternPlayerBox(player))
    } catch {
        chrsSetError(errorOut, error)
        return nil
    }
}

@_cdecl("chrs_engine_create_advanced_player")
public func chrs_engine_create_advanced_player(
    _ rawEngine: UnsafeMutableRawPointer?,
    _ rawPattern: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> UnsafeMutableRawPointer? {
    do {
        let player = try chrsEngineBox(rawEngine).engine.makeAdvancedPlayer(with: chrsPatternBox(rawPattern).pattern)
        return chrsRetain(AdvancedPatternPlayerBox(player))
    } catch {
        chrsSetError(errorOut, error)
        return nil
    }
}

@_cdecl("chrs_engine_register_audio_resource")
public func chrs_engine_register_audio_resource(
    _ rawEngine: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ optionsJSON: UnsafePointer<CChar>?,
    _ outResourceID: UnsafeMutablePointer<UInt64>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        guard let path = chrsString(path) else {
            throw chrsBridgeNSError(code: 53, message: "missing audio resource path")
        }
        let options = try chrsAudioResourceOptions(optionsJSON)
        let resourceID = try chrsEngineBox(rawEngine).engine.registerAudioResource(URL(fileURLWithPath: path), options: options)
        outResourceID?.pointee = UInt64(resourceID)
        return true
    } catch {
        chrsSetError(errorOut, error)
        return false
    }
}

@_cdecl("chrs_engine_unregister_audio_resource")
public func chrs_engine_unregister_audio_resource(
    _ rawEngine: UnsafeMutableRawPointer?,
    _ resourceID: UInt64,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        try chrsEngineBox(rawEngine).engine.unregisterAudioResource(Int(resourceID))
        return true
    } catch {
        chrsSetError(errorOut, error)
        return false
    }
}

@_cdecl("chrs_engine_play_pattern_from_url")
public func chrs_engine_play_pattern_from_url(
    _ rawEngine: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        guard let path = chrsString(path) else {
            throw chrsBridgeNSError(code: 54, message: "missing pattern file path")
        }
        try chrsEngineBox(rawEngine).engine.playPattern(from: URL(fileURLWithPath: path))
        return true
    } catch {
        chrsSetError(errorOut, error)
        return false
    }
}

@_cdecl("chrs_engine_play_pattern_from_data")
public func chrs_engine_play_pattern_from_data(
    _ rawEngine: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        let data: Data
        if length == 0 {
            data = Data()
        } else if let bytes {
            data = Data(bytes: bytes, count: length)
        } else {
            throw chrsBridgeNSError(code: 55, message: "missing pattern data bytes")
        }
        try chrsEngineBox(rawEngine).engine.playPattern(from: data)
        return true
    } catch {
        chrsSetError(errorOut, error)
        return false
    }
}

@_cdecl("chrs_engine_set_stopped_handler")
public func chrs_engine_set_stopped_handler(
    _ rawEngine: UnsafeMutableRawPointer?,
    _ callback: CHRSEngineStoppedHandler?,
    _ context: UnsafeMutableRawPointer?,
    _ dropContext: CHRSContextDrop?
) {
    let engineBox = chrsEngineBox(rawEngine)
    if let callback {
        let stoppedHandlerBox = EngineStoppedHandlerBox(
            callback: callback,
            context: context,
            dropContext: dropContext
        )
        engineBox.stoppedHandlerBox = stoppedHandlerBox
        engineBox.engine.stoppedHandler = { reason in
            stoppedHandlerBox.invoke(reason)
        }
    } else {
        engineBox.engine.stoppedHandler = { _ in }
        engineBox.stoppedHandlerBox = nil
    }
}

@_cdecl("chrs_engine_clear_stopped_handler")
public func chrs_engine_clear_stopped_handler(_ rawEngine: UnsafeMutableRawPointer?) {
    let engineBox = chrsEngineBox(rawEngine)
    engineBox.engine.stoppedHandler = { _ in }
    engineBox.stoppedHandlerBox = nil
}

@_cdecl("chrs_engine_set_reset_handler")
public func chrs_engine_set_reset_handler(
    _ rawEngine: UnsafeMutableRawPointer?,
    _ callback: CHRSEngineResetHandler?,
    _ context: UnsafeMutableRawPointer?,
    _ dropContext: CHRSContextDrop?
) {
    let engineBox = chrsEngineBox(rawEngine)
    if let callback {
        let resetHandlerBox = EngineResetHandlerBox(
            callback: callback,
            context: context,
            dropContext: dropContext
        )
        engineBox.resetHandlerBox = resetHandlerBox
        engineBox.engine.resetHandler = {
            resetHandlerBox.invoke()
        }
    } else {
        engineBox.engine.resetHandler = {}
        engineBox.resetHandlerBox = nil
    }
}

@_cdecl("chrs_engine_clear_reset_handler")
public func chrs_engine_clear_reset_handler(_ rawEngine: UnsafeMutableRawPointer?) {
    let engineBox = chrsEngineBox(rawEngine)
    engineBox.engine.resetHandler = {}
    engineBox.resetHandlerBox = nil
}

@_cdecl("chrs_engine_notify_when_players_finished")
public func chrs_engine_notify_when_players_finished(
    _ rawEngine: UnsafeMutableRawPointer?,
    _ callback: CHRSEngineFinishedHandler?,
    _ context: UnsafeMutableRawPointer?,
    _ dropContext: CHRSContextDrop?
) {
    let engineBox = chrsEngineBox(rawEngine)
    guard let callback else {
        engineBox.finishedHandlerBox = nil
        return
    }
    let finishedHandlerBox = EngineFinishedHandlerBox(
        callback: callback,
        context: context,
        dropContext: dropContext
    )
    engineBox.finishedHandlerBox = finishedHandlerBox
    engineBox.engine.notifyWhenPlayersFinished { [weak engineBox] error in
        let action = finishedHandlerBox.invoke(error)
        engineBox?.finishedHandlerBox = nil
        return action
    }
}
