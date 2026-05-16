import CoreHaptics
import Foundation

struct BridgeEvent: Decodable {
    let eventType: String
    let relativeTime: Double
    let duration: Double?
    let parameters: [BridgeEventParameter]?
    let audioResourceId: UInt64?
}

func chrsEventType(from raw: String) throws -> CHHapticEvent.EventType {
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
        throw chrsBridgeNSError(code: 30, message: "unknown event type: \(raw)")
    }
}

func chrsEventType(_ raw: UnsafePointer<CChar>?) throws -> CHHapticEvent.EventType {
    guard let raw = chrsString(raw) else {
        throw chrsBridgeNSError(code: 31, message: "missing event type")
    }
    return try chrsEventType(from: raw)
}

func chrsMakeEvent(_ event: BridgeEvent) throws -> CHHapticEvent {
    let parameters = try (event.parameters ?? []).map(chrsMakeEventParameter)
    if let audioResourceId = event.audioResourceId {
        if let duration = event.duration {
            return CHHapticEvent(
                audioResourceID: Int(audioResourceId),
                parameters: parameters,
                relativeTime: event.relativeTime,
                duration: duration
            )
        }
        return CHHapticEvent(
            audioResourceID: Int(audioResourceId),
            parameters: parameters,
            relativeTime: event.relativeTime
        )
    }
    if let duration = event.duration {
        return CHHapticEvent(
            eventType: try chrsEventType(from: event.eventType),
            parameters: parameters,
            relativeTime: event.relativeTime,
            duration: duration
        )
    }
    return CHHapticEvent(
        eventType: try chrsEventType(from: event.eventType),
        parameters: parameters,
        relativeTime: event.relativeTime
    )
}
