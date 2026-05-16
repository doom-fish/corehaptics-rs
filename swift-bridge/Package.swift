// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "CoreHapticsBridge",
    platforms: [
        .macOS(.v10_15),
    ],
    products: [
        .library(name: "CoreHapticsBridge", type: .static, targets: ["CoreHapticsBridge"]),
    ],
    targets: [
        .target(
            name: "CoreHapticsBridge",
            path: "Sources/CoreHapticsBridge",
            publicHeadersPath: "include"
        ),
    ]
)
