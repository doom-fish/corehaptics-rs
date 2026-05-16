import CoreHaptics
import Foundation

final class CapabilityBox: NSObject {
    let capability: any CHHapticDeviceCapability

    init(_ capability: any CHHapticDeviceCapability) {
        self.capability = capability
    }
}

func chrsCapabilityBox(_ raw: UnsafeMutableRawPointer?) -> CapabilityBox {
    chrsBorrow(raw, as: CapabilityBox.self)
}

@_cdecl("chrs_capabilities_for_hardware")
public func chrs_capabilities_for_hardware() -> UnsafeMutableRawPointer? {
    chrsRetain(CapabilityBox(CHHapticEngine.capabilitiesForHardware()))
}

@_cdecl("chrs_capability_supports_haptics")
public func chrs_capability_supports_haptics(_ capability: UnsafeMutableRawPointer?) -> Bool {
    chrsCapabilityBox(capability).capability.supportsHaptics
}

@_cdecl("chrs_capability_supports_audio")
public func chrs_capability_supports_audio(_ capability: UnsafeMutableRawPointer?) -> Bool {
    chrsCapabilityBox(capability).capability.supportsAudio
}

@_cdecl("chrs_capability_event_parameter_attributes")
public func chrs_capability_event_parameter_attributes(
    _ capability: UnsafeMutableRawPointer?,
    _ parameterID: UnsafePointer<CChar>?,
    _ eventType: UnsafePointer<CChar>?,
    _ outMin: UnsafeMutablePointer<Float>?,
    _ outMax: UnsafeMutablePointer<Float>?,
    _ outDefault: UnsafeMutablePointer<Float>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        let attributes = try chrsCapabilityBox(capability).capability.attributes(
            forEventParameter: chrsEventParameterID(parameterID),
            eventType: chrsEventType(eventType)
        )
        outMin?.pointee = attributes.minValue
        outMax?.pointee = attributes.maxValue
        outDefault?.pointee = attributes.defaultValue
        return true
    } catch {
        chrsSetError(errorOut, error)
        return false
    }
}

@_cdecl("chrs_capability_dynamic_parameter_attributes")
public func chrs_capability_dynamic_parameter_attributes(
    _ capability: UnsafeMutableRawPointer?,
    _ parameterID: UnsafePointer<CChar>?,
    _ outMin: UnsafeMutablePointer<Float>?,
    _ outMax: UnsafeMutablePointer<Float>?,
    _ outDefault: UnsafeMutablePointer<Float>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        let attributes = try chrsCapabilityBox(capability).capability.attributes(
            forDynamicParameter: chrsDynamicParameterID(parameterID)
        )
        outMin?.pointee = attributes.minValue
        outMax?.pointee = attributes.maxValue
        outDefault?.pointee = attributes.defaultValue
        return true
    } catch {
        chrsSetError(errorOut, error)
        return false
    }
}
