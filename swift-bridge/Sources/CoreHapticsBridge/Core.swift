import CoreHaptics
import Dispatch
import Foundation

public typealias CHRSContextDrop = @convention(c) (UnsafeMutableRawPointer?) -> Void
public typealias CHRSEngineStoppedHandler = @convention(c) (UnsafeMutableRawPointer?, Int32) -> Void
public typealias CHRSEngineResetHandler = @convention(c) (UnsafeMutableRawPointer?) -> Void
public typealias CHRSEngineFinishedHandler = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Int32
public typealias CHRSEngineCompletionHandler = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void
public typealias CHRSAdvancedPlayerCompletionHandler = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void

@inline(__always)
func chrsRetain(_ object: some AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func chrsBorrow<T: AnyObject>(_ raw: UnsafeMutableRawPointer?, as type: T.Type = T.self) -> T {
    let bound = raw!.assumingMemoryBound(to: T.self)
    return Unmanaged<T>.fromOpaque(UnsafeRawPointer(bound)).takeUnretainedValue()
}

@inline(__always)
func chrsRelease(_ raw: UnsafeMutableRawPointer?) {
    guard let raw else { return }
    Unmanaged<NSObject>.fromOpaque(UnsafeRawPointer(raw.assumingMemoryBound(to: NSObject.self))).release()
}

@inline(__always)
func chrsNSError(_ raw: UnsafeMutableRawPointer?) -> NSError {
    chrsBorrow(raw, as: NSError.self)
}

func chrsSetError(_ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?, _ error: Error) {
    errorOut?.pointee = chrsRetain(error as NSError)
}

func chrsCString(_ value: String) -> UnsafeMutablePointer<CChar>? {
    strdup(value)
}

func chrsString(_ raw: UnsafePointer<CChar>?) -> String? {
    guard let raw else { return nil }
    return String(cString: raw)
}

func chrsBridgeNSError(code: Int, message: String) -> NSError {
    NSError(
        domain: "CoreHapticsBridge",
        code: code,
        userInfo: [NSLocalizedDescriptionKey: message]
    )
}

func chrsDecodeJSON<T: Decodable>(_ raw: UnsafePointer<CChar>?, as type: T.Type) throws -> T {
    guard let string = chrsString(raw) else {
        throw chrsBridgeNSError(code: 1, message: "missing JSON payload")
    }
    return try JSONDecoder().decode(T.self, from: Data(string.utf8))
}

func chrsEncodeDictionaryJSON(_ dictionary: [CHHapticPattern.Key: Any]) throws -> String {
    let data = try JSONSerialization.data(withJSONObject: dictionary as NSDictionary, options: [.sortedKeys])
    guard let string = String(data: data, encoding: .utf8) else {
        throw chrsBridgeNSError(code: 2, message: "failed to encode pattern dictionary as UTF-8")
    }
    return string
}

@_cdecl("chrs_object_retain")
public func chrs_object_retain(_ obj: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let obj else { return nil }
    return chrsRetain(chrsBorrow(obj, as: NSObject.self))
}

@_cdecl("chrs_object_release")
public func chrs_object_release(_ obj: UnsafeMutableRawPointer?) {
    chrsRelease(obj)
}

@_cdecl("chrs_string_free")
public func chrs_string_free(_ ptr: UnsafeMutablePointer<CChar>?) {
    free(ptr)
}

@_cdecl("chrs_error_code")
public func chrs_error_code(_ error: UnsafeMutableRawPointer?) -> Int {
    chrsNSError(error).code
}

@_cdecl("chrs_error_domain")
public func chrs_error_domain(_ error: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    chrsCString(chrsNSError(error).domain)
}

@_cdecl("chrs_error_description")
public func chrs_error_description(_ error: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    chrsCString(chrsNSError(error).localizedDescription)
}
