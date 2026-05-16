import CoreHaptics
import Foundation

struct BridgeParameterCurveControlPoint: Decodable {
    let relativeTime: Double
    let value: Float
}

struct BridgeParameterCurve: Decodable {
    let parameterId: String
    let relativeTime: Double
    let controlPoints: [BridgeParameterCurveControlPoint]
}

func chrsMakeControlPoint(_ controlPoint: BridgeParameterCurveControlPoint) -> CHHapticParameterCurve.ControlPoint {
    CHHapticParameterCurve.ControlPoint(relativeTime: controlPoint.relativeTime, value: controlPoint.value)
}

func chrsMakeParameterCurve(_ parameterCurve: BridgeParameterCurve) throws -> CHHapticParameterCurve {
    CHHapticParameterCurve(
        parameterID: try chrsDynamicParameterID(from: parameterCurve.parameterId),
        controlPoints: parameterCurve.controlPoints.map(chrsMakeControlPoint),
        relativeTime: parameterCurve.relativeTime
    )
}
