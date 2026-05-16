use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    println!("cargo:rerun-if-env-changed=DEVELOPER_DIR");

    if env::var("DOCS_RS").is_ok() {
        return;
    }

    println!("cargo:rustc-link-lib=framework=CoreHaptics");
    println!("cargo:rustc-link-lib=framework=Foundation");

    let swift_dir = "swift-bridge";
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR");
    let swift_build_dir = format!("{out_dir}/swift-build");
    println!("cargo:rerun-if-changed={swift_dir}");

    if let Ok(output) = Command::new("swiftlint")
        .args(["lint"])
        .current_dir(swift_dir)
        .output()
    {
        if !output.status.success() {
            eprintln!(
                "SwiftLint warnings:\n{}",
                String::from_utf8_lossy(&output.stdout)
            );
        }
    }

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let swift_triple = match target_arch.as_str() {
        "x86_64" => "x86_64-apple-macosx",
        "aarch64" => "arm64-apple-macosx",
        other => panic!("corehaptics: unsupported target arch '{other}'"),
    };

    let output = Command::new("swift")
        .args([
            "build",
            "-c",
            "release",
            "--triple",
            swift_triple,
            "--package-path",
            swift_dir,
            "--scratch-path",
            &swift_build_dir,
        ])
        .output()
        .expect("Failed to build Swift bridge");

    if !output.status.success() {
        eprintln!(
            "Swift build STDOUT:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
        eprintln!(
            "Swift build STDERR:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        panic!("Swift bridge build failed");
    }

    println!("cargo:rustc-link-search=native={swift_build_dir}/release");
    println!("cargo:rustc-link-lib=static=CoreHapticsBridge");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

    match Command::new("xcode-select").arg("-p").output() {
        Ok(output) if output.status.success() => {
            let xcode_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let new_runtime =
                format!("{xcode_path}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx");
            println!("cargo:rustc-link-search=native={new_runtime}");
            println!("cargo:rustc-link-arg=-Wl,-rpath,{new_runtime}");

            let old_runtime = format!(
                "{xcode_path}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift-5.5/macosx"
            );
            println!("cargo:rustc-link-search=native={old_runtime}");
            println!("cargo:rustc-link-arg=-Wl,-rpath,{old_runtime}");
        }
        Ok(output) => {
            println!(
                "cargo:warning=`xcode-select -p` exited non-zero (status={:?}); the Swift runtime rpaths were not added.",
                output.status.code()
            );
        }
        Err(error) => {
            println!(
                "cargo:warning=`xcode-select` could not be invoked ({error}); the Swift runtime rpaths were not added."
            );
        }
    }
}
