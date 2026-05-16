import CoreHaptics
import Dispatch
import Foundation

private final class CapabilityBox: NSObject {
    let capability: any CHHapticDeviceCapability

    init(_ capability: any CHHapticDeviceCapability) {
        self.capability = capability
    }
}

private final class PlayerBox: NSObject {
    let player: any CHHapticPatternPlayer

    init(_ player: any CHHapticPatternPlayer) {
        self.player = player
    }
}

private struct BridgePattern: Decodable {
    let events: [BridgeEvent]
    let dynamicParameters: [BridgeDynamicParameter]?
}

private struct BridgeEvent: Decodable {
    let eventType: String
    let relativeTime: Double
    let duration: Double?
    let parameters: [BridgeEventParameter]?
}

private struct BridgeEventParameter: Decodable {
    let parameterId: String
    let value: Float
}

private struct BridgeDynamicParameter: Decodable {
    let parameterId: String
    let value: Float
    let relativeTime: Double
}

private func retainObject(_ object: AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

private func releaseObject(_ raw: UnsafeMutableRawPointer?) {
    guard let raw else { return }
    Unmanaged<AnyObject>.fromOpaque(raw).release()
}

private func nsError(_ error: UnsafeMutableRawPointer?) -> NSError {
    Unmanaged<NSError>.fromOpaque(error!).takeUnretainedValue()
}

private func capabilityBox(_ raw: UnsafeMutableRawPointer?) -> CapabilityBox {
    Unmanaged<CapabilityBox>.fromOpaque(raw!).takeUnretainedValue()
}

private func engine(_ raw: UnsafeMutableRawPointer?) -> CHHapticEngine {
    Unmanaged<CHHapticEngine>.fromOpaque(raw!).takeUnretainedValue()
}

private func pattern(_ raw: UnsafeMutableRawPointer?) -> CHHapticPattern {
    Unmanaged<CHHapticPattern>.fromOpaque(raw!).takeUnretainedValue()
}

private func playerBox(_ raw: UnsafeMutableRawPointer?) -> PlayerBox {
    Unmanaged<PlayerBox>.fromOpaque(raw!).takeUnretainedValue()
}

private func setError(_ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?, _ error: Error) {
    errorOut?.pointee = retainObject(error as NSError)
}

private func dupCString(_ value: String) -> UnsafeMutablePointer<CChar>? {
    strdup(value)
}

private func decodeString(_ raw: UnsafePointer<CChar>?) -> String? {
    guard let raw else { return nil }
    return String(cString: raw)
}

private func eventType(from raw: String) throws -> CHHapticEvent.EventType {
    switch raw {
    case "hapticTransient":
        return .hapticTransient
    case "hapticContinuous":
        return .hapticContinuous
    case "audioContinuous":
        return .audioContinuous
    case "audioCustom":
        return .audioCustom
    default:
        throw NSError(domain: "CoreHapticsBridge", code: 1, userInfo: [NSLocalizedDescriptionKey: "unknown event type: \(raw)"])
    }
}

private func eventParameterId(from raw: String) throws -> CHHapticEvent.ParameterID {
    switch raw {
    case "hapticIntensity":
        return .hapticIntensity
    case "hapticSharpness":
        return .hapticSharpness
    case "attackTime":
        return .attackTime
    case "decayTime":
        return .decayTime
    case "releaseTime":
        return .releaseTime
    case "sustained":
        return .sustained
    case "audioVolume":
        return .audioVolume
    case "audioPitch":
        return .audioPitch
    case "audioPan":
        return .audioPan
    case "audioBrightness":
        return .audioBrightness
    default:
        throw NSError(domain: "CoreHapticsBridge", code: 2, userInfo: [NSLocalizedDescriptionKey: "unknown parameter id: \(raw)"])
    }
}

private func dynamicParameterId(from raw: String) throws -> CHHapticDynamicParameter.ID {
    switch raw {
    case "hapticIntensityControl":
        return .hapticIntensityControl
    case "hapticSharpnessControl":
        return .hapticSharpnessControl
    case "hapticAttackTimeControl":
        return .hapticAttackTimeControl
    case "hapticDecayTimeControl":
        return .hapticDecayTimeControl
    case "hapticReleaseTimeControl":
        return .hapticReleaseTimeControl
    case "audioVolumeControl":
        return .audioVolumeControl
    case "audioPanControl":
        return .audioPanControl
    case "audioBrightnessControl":
        return .audioBrightnessControl
    case "audioPitchControl":
        return .audioPitchControl
    case "audioAttackTimeControl":
        return .audioAttackTimeControl
    case "audioDecayTimeControl":
        return .audioDecayTimeControl
    case "audioReleaseTimeControl":
        return .audioReleaseTimeControl
    default:
        throw NSError(domain: "CoreHapticsBridge", code: 3, userInfo: [NSLocalizedDescriptionKey: "unknown dynamic parameter id: \(raw)"])
    }
}

private func makeEventParameter(_ parameter: BridgeEventParameter) throws -> CHHapticEventParameter {
    CHHapticEventParameter(parameterID: try eventParameterId(from: parameter.parameterId), value: parameter.value)
}

private func makeDynamicParameter(_ parameter: BridgeDynamicParameter) throws -> CHHapticDynamicParameter {
    CHHapticDynamicParameter(
        parameterID: try dynamicParameterId(from: parameter.parameterId),
        value: parameter.value,
        relativeTime: parameter.relativeTime
    )
}

private func makeEvent(_ event: BridgeEvent) throws -> CHHapticEvent {
    let parameters = try (event.parameters ?? []).map(makeEventParameter)
    if let duration = event.duration {
        return CHHapticEvent(
            eventType: try eventType(from: event.eventType),
            parameters: parameters,
            relativeTime: event.relativeTime,
            duration: duration
        )
    }
    return CHHapticEvent(
        eventType: try eventType(from: event.eventType),
        parameters: parameters,
        relativeTime: event.relativeTime
    )
}

@_cdecl("chrs_object_retain")
public func chrs_object_retain(_ obj: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let obj else { return nil }
    return retainObject(Unmanaged<AnyObject>.fromOpaque(obj).takeUnretainedValue())
}

@_cdecl("chrs_object_release")
public func chrs_object_release(_ obj: UnsafeMutableRawPointer?) {
    releaseObject(obj)
}

@_cdecl("chrs_string_free")
public func chrs_string_free(_ ptr: UnsafeMutablePointer<CChar>?) {
    free(ptr)
}

@_cdecl("chrs_error_code")
public func chrs_error_code(_ error: UnsafeMutableRawPointer?) -> Int {
    nsError(error).code
}

@_cdecl("chrs_error_domain")
public func chrs_error_domain(_ error: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    dupCString(nsError(error).domain)
}

@_cdecl("chrs_error_description")
public func chrs_error_description(_ error: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    dupCString(nsError(error).localizedDescription)
}

@_cdecl("chrs_capabilities_for_hardware")
public func chrs_capabilities_for_hardware() -> UnsafeMutableRawPointer? {
    retainObject(CapabilityBox(CHHapticEngine.capabilitiesForHardware()))
}

@_cdecl("chrs_capability_supports_haptics")
public func chrs_capability_supports_haptics(_ capability: UnsafeMutableRawPointer?) -> Bool {
    capabilityBox(capability).capability.supportsHaptics
}

@_cdecl("chrs_capability_supports_audio")
public func chrs_capability_supports_audio(_ capability: UnsafeMutableRawPointer?) -> Bool {
    capabilityBox(capability).capability.supportsAudio
}

@_cdecl("chrs_engine_create")
public func chrs_engine_create(_ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?) -> UnsafeMutableRawPointer? {
    do {
        let engine = try CHHapticEngine()
        return retainObject(engine)
    } catch {
        setError(errorOut, error)
        return nil
    }
}

@_cdecl("chrs_engine_start")
public func chrs_engine_start(
    _ rawEngine: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        try engine(rawEngine).start()
        return true
    } catch {
        setError(errorOut, error)
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
    engine(rawEngine).stop { error in
        capturedError = error
        semaphore.signal()
    }
    semaphore.wait()
    if let capturedError {
        setError(errorOut, capturedError)
        return false
    }
    return true
}

@_cdecl("chrs_engine_current_time")
public func chrs_engine_current_time(_ rawEngine: UnsafeMutableRawPointer?) -> Double {
    engine(rawEngine).currentTime
}

@_cdecl("chrs_engine_auto_shutdown_enabled")
public func chrs_engine_auto_shutdown_enabled(_ rawEngine: UnsafeMutableRawPointer?) -> Bool {
    engine(rawEngine).isAutoShutdownEnabled
}

@_cdecl("chrs_engine_set_auto_shutdown_enabled")
public func chrs_engine_set_auto_shutdown_enabled(_ rawEngine: UnsafeMutableRawPointer?, _ enabled: Bool) {
    engine(rawEngine).isAutoShutdownEnabled = enabled
}

@_cdecl("chrs_engine_create_player")
public func chrs_engine_create_player(
    _ rawEngine: UnsafeMutableRawPointer?,
    _ rawPattern: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> UnsafeMutableRawPointer? {
    do {
        let player = try engine(rawEngine).makePlayer(with: pattern(rawPattern))
        return retainObject(PlayerBox(player))
    } catch {
        setError(errorOut, error)
        return nil
    }
}

@_cdecl("chrs_pattern_create")
public func chrs_pattern_create(
    _ patternJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> UnsafeMutableRawPointer? {
    do {
        guard let json = decodeString(patternJSON) else {
            throw NSError(domain: "CoreHapticsBridge", code: 4, userInfo: [NSLocalizedDescriptionKey: "pattern JSON was null"])
        }
        let data = Data(json.utf8)
        let decoded = try JSONDecoder().decode(BridgePattern.self, from: data)
        let events = try decoded.events.map(makeEvent)
        let dynamicParameters = try (decoded.dynamicParameters ?? []).map(makeDynamicParameter)
        let pattern = try CHHapticPattern(events: events, parameters: dynamicParameters)
        return retainObject(pattern)
    } catch {
        setError(errorOut, error)
        return nil
    }
}

@_cdecl("chrs_pattern_duration")
public func chrs_pattern_duration(_ rawPattern: UnsafeMutableRawPointer?) -> Double {
    pattern(rawPattern).duration
}

@_cdecl("chrs_player_start")
public func chrs_player_start(
    _ rawPlayer: UnsafeMutableRawPointer?,
    _ time: Double,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        try playerBox(rawPlayer).player.start(atTime: time)
        return true
    } catch {
        setError(errorOut, error)
        return false
    }
}

@_cdecl("chrs_player_stop")
public func chrs_player_stop(
    _ rawPlayer: UnsafeMutableRawPointer?,
    _ time: Double,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        try playerBox(rawPlayer).player.stop(atTime: time)
        return true
    } catch {
        setError(errorOut, error)
        return false
    }
}

@_cdecl("chrs_player_cancel")
public func chrs_player_cancel(
    _ rawPlayer: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        try playerBox(rawPlayer).player.cancel()
        return true
    } catch {
        setError(errorOut, error)
        return false
    }
}
