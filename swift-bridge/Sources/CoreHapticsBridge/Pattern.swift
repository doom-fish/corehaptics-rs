import CoreHaptics
import Foundation

private struct BridgePatternEnvelope: Decodable {
    let events: [BridgeEvent]
    let dynamicParameters: [BridgeDynamicParameter]?
    let parameterCurves: [BridgeParameterCurve]?
}

final class PatternBox: NSObject {
    let pattern: CHHapticPattern

    init(_ pattern: CHHapticPattern) {
        self.pattern = pattern
    }
}

func chrsPatternBox(_ raw: UnsafeMutableRawPointer?) -> PatternBox {
    chrsBorrow(raw, as: PatternBox.self)
}

private func chrsJSONObject(_ value: Any) throws -> Any {
    if let dictionary = value as? [String: Any] {
        return try chrsPatternDictionary(dictionary)
    }
    if let array = value as? [Any] {
        return try array.map(chrsJSONObject)
    }
    return value
}

private func chrsPatternDictionary(_ dictionary: [String: Any]) throws -> [CHHapticPattern.Key: Any] {
    var converted: [CHHapticPattern.Key: Any] = [:]
    for (key, value) in dictionary {
        converted[NSString(string: key) as CHHapticPattern.Key] = try chrsJSONObject(value)
    }
    return converted
}

private func chrsPatternDictionary(from raw: UnsafePointer<CChar>?) throws -> [CHHapticPattern.Key: Any] {
    guard let string = chrsString(raw) else {
        throw chrsBridgeNSError(code: 40, message: "missing pattern dictionary JSON")
    }
    let object = try JSONSerialization.jsonObject(with: Data(string.utf8), options: [])
    guard let dictionary = object as? [String: Any] else {
        throw chrsBridgeNSError(code: 41, message: "pattern dictionary JSON must decode to an object")
    }
    return try chrsPatternDictionary(dictionary)
}

@_cdecl("chrs_pattern_create")
public func chrs_pattern_create(
    _ patternJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> UnsafeMutableRawPointer? {
    do {
        let envelope = try chrsDecodeJSON(patternJSON, as: BridgePatternEnvelope.self)
        let events = try envelope.events.map(chrsMakeEvent)
        if let parameterCurves = envelope.parameterCurves, !parameterCurves.isEmpty {
            let curves = try parameterCurves.map(chrsMakeParameterCurve)
            return chrsRetain(PatternBox(try CHHapticPattern(events: events, parameterCurves: curves)))
        }
        let dynamicParameters = try (envelope.dynamicParameters ?? []).map(chrsMakeDynamicParameter)
        return chrsRetain(PatternBox(try CHHapticPattern(events: events, parameters: dynamicParameters)))
    } catch {
        chrsSetError(errorOut, error)
        return nil
    }
}

@_cdecl("chrs_pattern_create_from_dictionary_json")
public func chrs_pattern_create_from_dictionary_json(
    _ patternJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> UnsafeMutableRawPointer? {
    do {
        let dictionary = try chrsPatternDictionary(from: patternJSON)
        return chrsRetain(PatternBox(try CHHapticPattern(dictionary: dictionary)))
    } catch {
        chrsSetError(errorOut, error)
        return nil
    }
}

@_cdecl("chrs_pattern_create_from_ahap_file")
public func chrs_pattern_create_from_ahap_file(
    _ path: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> UnsafeMutableRawPointer? {
    do {
        guard let path = chrsString(path) else {
            throw chrsBridgeNSError(code: 42, message: "missing AHAP path")
        }
        guard #available(macOS 13.0, *) else {
            throw chrsBridgeNSError(code: 43, message: "CHHapticPattern(contentsOf:) requires macOS 13.0")
        }
        return chrsRetain(PatternBox(try CHHapticPattern(contentsOf: URL(fileURLWithPath: path))))
    } catch {
        chrsSetError(errorOut, error)
        return nil
    }
}

@_cdecl("chrs_pattern_export_dictionary_json")
public func chrs_pattern_export_dictionary_json(
    _ rawPattern: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let dictionary = try chrsPatternBox(rawPattern).pattern.exportDictionary()
        return chrsCString(try chrsEncodeDictionaryJSON(dictionary))
    } catch {
        chrsSetError(errorOut, error)
        return nil
    }
}

@_cdecl("chrs_pattern_duration")
public func chrs_pattern_duration(_ rawPattern: UnsafeMutableRawPointer?) -> Double {
    chrsPatternBox(rawPattern).pattern.duration
}
