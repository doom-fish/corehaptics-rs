import CoreHaptics
import Foundation

struct BridgeDynamicParameter: Decodable {
    let parameterId: String
    let value: Float
    let relativeTime: Double
}

func chrsDynamicParameterID(from raw: String) throws -> CHHapticDynamicParameter.ID {
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
        throw chrsBridgeNSError(code: 20, message: "unknown dynamic parameter id: \(raw)")
    }
}

func chrsDynamicParameterID(_ raw: UnsafePointer<CChar>?) throws -> CHHapticDynamicParameter.ID {
    guard let raw = chrsString(raw) else {
        throw chrsBridgeNSError(code: 21, message: "missing dynamic parameter id")
    }
    return try chrsDynamicParameterID(from: raw)
}

func chrsMakeDynamicParameter(_ parameter: BridgeDynamicParameter) throws -> CHHapticDynamicParameter {
    CHHapticDynamicParameter(
        parameterID: try chrsDynamicParameterID(from: parameter.parameterId),
        value: parameter.value,
        relativeTime: parameter.relativeTime
    )
}
