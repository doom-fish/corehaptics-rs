import CoreHaptics
import Foundation

final class AdvancedPlayerCompletionBox {
    let callback: CHRSAdvancedPlayerCompletionHandler
    let context: UnsafeMutableRawPointer?
    let dropContext: CHRSContextDrop?

    init(
        callback: @escaping CHRSAdvancedPlayerCompletionHandler,
        context: UnsafeMutableRawPointer?,
        dropContext: CHRSContextDrop?
    ) {
        self.callback = callback
        self.context = context
        self.dropContext = dropContext
    }

    func invoke(_ error: Error?) {
        let errorPointer = error.map { chrsRetain($0 as NSError) }
        callback(context, errorPointer)
    }

    deinit {
        dropContext?(context)
    }
}

final class AdvancedPatternPlayerBox: PatternPlayerBox {
    private(set) var completionBox: AdvancedPlayerCompletionBox?
    private let boxedAdvancedPlayer: any CHHapticAdvancedPatternPlayer

    var advancedPlayer: any CHHapticAdvancedPatternPlayer {
        boxedAdvancedPlayer
    }

    init(_ player: any CHHapticAdvancedPatternPlayer) {
        boxedAdvancedPlayer = player
        super.init(player)
        advancedPlayer.completionHandler = { _ in }
    }

    func setCompletionHandler(_ completionBox: AdvancedPlayerCompletionBox?) {
        self.completionBox = completionBox
        if let completionBox {
            advancedPlayer.completionHandler = { error in
                completionBox.invoke(error)
            }
        } else {
            advancedPlayer.completionHandler = { _ in }
        }
    }
}

func chrsAdvancedPatternPlayerBox(_ raw: UnsafeMutableRawPointer?) -> AdvancedPatternPlayerBox {
    chrsBorrow(raw, as: AdvancedPatternPlayerBox.self)
}

@_cdecl("chrs_advanced_player_pause")
public func chrs_advanced_player_pause(
    _ rawPlayer: UnsafeMutableRawPointer?,
    _ time: Double,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        try chrsAdvancedPatternPlayerBox(rawPlayer).advancedPlayer.pause(atTime: time)
        return true
    } catch {
        chrsSetError(errorOut, error)
        return false
    }
}

@_cdecl("chrs_advanced_player_resume")
public func chrs_advanced_player_resume(
    _ rawPlayer: UnsafeMutableRawPointer?,
    _ time: Double,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        try chrsAdvancedPatternPlayerBox(rawPlayer).advancedPlayer.resume(atTime: time)
        return true
    } catch {
        chrsSetError(errorOut, error)
        return false
    }
}

@_cdecl("chrs_advanced_player_seek_to_offset")
public func chrs_advanced_player_seek_to_offset(
    _ rawPlayer: UnsafeMutableRawPointer?,
    _ offset: Double,
    _ errorOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    do {
        try chrsAdvancedPatternPlayerBox(rawPlayer).advancedPlayer.seek(toOffset: offset)
        return true
    } catch {
        chrsSetError(errorOut, error)
        return false
    }
}

@_cdecl("chrs_advanced_player_loop_enabled")
public func chrs_advanced_player_loop_enabled(_ rawPlayer: UnsafeMutableRawPointer?) -> Bool {
    chrsAdvancedPatternPlayerBox(rawPlayer).advancedPlayer.loopEnabled
}

@_cdecl("chrs_advanced_player_set_loop_enabled")
public func chrs_advanced_player_set_loop_enabled(_ rawPlayer: UnsafeMutableRawPointer?, _ enabled: Bool) {
    chrsAdvancedPatternPlayerBox(rawPlayer).advancedPlayer.loopEnabled = enabled
}

@_cdecl("chrs_advanced_player_loop_end")
public func chrs_advanced_player_loop_end(_ rawPlayer: UnsafeMutableRawPointer?) -> Double {
    chrsAdvancedPatternPlayerBox(rawPlayer).advancedPlayer.loopEnd
}

@_cdecl("chrs_advanced_player_set_loop_end")
public func chrs_advanced_player_set_loop_end(_ rawPlayer: UnsafeMutableRawPointer?, _ loopEnd: Double) {
    chrsAdvancedPatternPlayerBox(rawPlayer).advancedPlayer.loopEnd = loopEnd
}

@_cdecl("chrs_advanced_player_playback_rate")
public func chrs_advanced_player_playback_rate(_ rawPlayer: UnsafeMutableRawPointer?) -> Float {
    chrsAdvancedPatternPlayerBox(rawPlayer).advancedPlayer.playbackRate
}

@_cdecl("chrs_advanced_player_set_playback_rate")
public func chrs_advanced_player_set_playback_rate(_ rawPlayer: UnsafeMutableRawPointer?, _ playbackRate: Float) {
    chrsAdvancedPatternPlayerBox(rawPlayer).advancedPlayer.playbackRate = playbackRate
}

@_cdecl("chrs_advanced_player_set_completion_handler")
public func chrs_advanced_player_set_completion_handler(
    _ rawPlayer: UnsafeMutableRawPointer?,
    _ callback: CHRSAdvancedPlayerCompletionHandler?,
    _ context: UnsafeMutableRawPointer?,
    _ dropContext: CHRSContextDrop?
) {
    guard let callback else {
        chrsAdvancedPatternPlayerBox(rawPlayer).setCompletionHandler(nil)
        return
    }
    let completionBox = AdvancedPlayerCompletionBox(
        callback: callback,
        context: context,
        dropContext: dropContext
    )
    chrsAdvancedPatternPlayerBox(rawPlayer).setCompletionHandler(completionBox)
}

@_cdecl("chrs_advanced_player_clear_completion_handler")
public func chrs_advanced_player_clear_completion_handler(_ rawPlayer: UnsafeMutableRawPointer?) {
    chrsAdvancedPatternPlayerBox(rawPlayer).setCompletionHandler(nil)
}
