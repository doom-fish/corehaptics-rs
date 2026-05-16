import CoreHaptics
import Foundation

class PatternPlayerBox: NSObject {
    let player: any CHHapticPatternPlayer

    init(_ player: any CHHapticPatternPlayer) {
        self.player = player
    }
}

func chrsPatternPlayerBox(_ raw: UnsafeMutableRawPointer?) -> PatternPlayerBox {
    chrsBorrow(raw, as: PatternPlayerBox.self)
}

@_cdecl("chrs_player_start")
public func chrs_player_start(
    _ rawPlayer: UnsafeMutableRawPointer?,
    _ time: Double,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        try chrsPatternPlayerBox(rawPlayer).player.start(atTime: time)
        return true
    } catch {
        chrsSetError(errorOut, error)
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
        try chrsPatternPlayerBox(rawPlayer).player.stop(atTime: time)
        return true
    } catch {
        chrsSetError(errorOut, error)
        return false
    }
}

@_cdecl("chrs_player_cancel")
public func chrs_player_cancel(
    _ rawPlayer: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        try chrsPatternPlayerBox(rawPlayer).player.cancel()
        return true
    } catch {
        chrsSetError(errorOut, error)
        return false
    }
}

@_cdecl("chrs_player_send_parameters")
public func chrs_player_send_parameters(
    _ rawPlayer: UnsafeMutableRawPointer?,
    _ parametersJSON: UnsafePointer<CChar>?,
    _ time: Double,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        let parameters = try chrsDecodeJSON(parametersJSON, as: [BridgeDynamicParameter].self)
            .map(chrsMakeDynamicParameter)
        try chrsPatternPlayerBox(rawPlayer).player.sendParameters(parameters, atTime: time)
        return true
    } catch {
        chrsSetError(errorOut, error)
        return false
    }
}

@_cdecl("chrs_player_schedule_parameter_curve")
public func chrs_player_schedule_parameter_curve(
    _ rawPlayer: UnsafeMutableRawPointer?,
    _ parameterCurveJSON: UnsafePointer<CChar>?,
    _ time: Double,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        let parameterCurve = try chrsMakeParameterCurve(chrsDecodeJSON(parameterCurveJSON, as: BridgeParameterCurve.self))
        try chrsPatternPlayerBox(rawPlayer).player.scheduleParameterCurve(parameterCurve, atTime: time)
        return true
    } catch {
        chrsSetError(errorOut, error)
        return false
    }
}

@_cdecl("chrs_player_is_muted")
public func chrs_player_is_muted(_ rawPlayer: UnsafeMutableRawPointer?) -> Bool {
    chrsPatternPlayerBox(rawPlayer).player.isMuted
}

@_cdecl("chrs_player_set_muted")
public func chrs_player_set_muted(_ rawPlayer: UnsafeMutableRawPointer?, _ muted: Bool) {
    chrsPatternPlayerBox(rawPlayer).player.isMuted = muted
}
