import CoreHaptics
import Foundation

struct BridgeEventParameter: Decodable {
    let parameterId: String
    let value: Float
}

func chrsEventParameterID(from raw: String) throws -> CHHapticEvent.ParameterID {
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
        throw chrsBridgeNSError(code: 10, message: "unknown event parameter id: \(raw)")
    }
}

func chrsEventParameterID(_ raw: UnsafePointer<CChar>?) throws -> CHHapticEvent.ParameterID {
    guard let raw = chrsString(raw) else {
        throw chrsBridgeNSError(code: 11, message: "missing event parameter id")
    }
    return try chrsEventParameterID(from: raw)
}

func chrsMakeEventParameter(_ parameter: BridgeEventParameter) throws -> CHHapticEventParameter {
    CHHapticEventParameter(parameterID: try chrsEventParameterID(from: parameter.parameterId), value: parameter.value)
}
