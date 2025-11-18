use std::{env, path::Path};

fn main() {
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    let bridges = [
        "src/lib.rs",
        "src/utils.rs",
        "src/juce_core/string.rs",
        "src/juce_core/bigint.rs",
        "src/juce_core/array.rs",
        "src/juce_core/system.rs",
        "src/juce_audio_basics/filters.rs",
        "src/juce_audio_basics/buffer.rs",
        "src/juce_audio_devices/mod.rs",
        "src/juce_audio_devices/device_manager.rs",
        "src/juce_audio_devices/device.rs",
        "src/juce_audio_devices/device_type.rs",
        "src/juce_audio_devices/device_callback.rs",
    ];
    for bridge in bridges.iter() {
        let _ = cxx_build::bridge(bridge);
        println!("cargo:rerun-if-changed={bridge}");
    }

    let mut cmake = cmake::Config::new("bridge");
    cmake.build_target("cxx-juce");

    cmake.define(
        "CXX_JUCE_BRIDGE_FILES",
        bridges
            .iter()
            .map(|bridge| format!("../target/cxxbridge/cxx-juce/{}.cc", bridge))
            .collect::<Vec<_>>()
            .join(";"),
    );

    if cfg!(feature = "asio") {
        cmake.define("CXX_JUCE_USE_ASIO", "ON");

        if let Ok(path) = env::var("CXX_JUCE_ASIO_SDK_DIR") {
            if Path::new(&path).join("common").join("iasiodrv.h").exists() {
                cmake.define("CXX_JUCE_ASIO_SDK_DIR", path);
            } else {
                panic!("CXX_JUCE_ASIO_SDK_DIR is set to '{path}' which is not a valid path to the ASIO SDK");
            }
        } else {
            panic!("CXX_JUCE_ASIO_SDK_DIR is not set");
        }
    } else {
        cmake.define("CXX_JUCE_USE_ASIO", "OFF");
    }

    if cfg!(target_os = "windows") && cmake.get_profile() == "Debug" {
        cmake.profile("RelWithDebInfo");
    }

    let destination = cmake.build();

    println!("cargo:rerun-if-changed=bridge");
    println!("cargo:rerun-if-env-changed=CXX_JUCE_ASIO_SDK_DIR");

    if cfg!(target_os = "windows") {
        println!(
            "cargo:rustc-link-search=native={}/build/{}",
            destination.display(),
            cmake.get_profile()
        );
    } else {
        println!(
            "cargo:rustc-link-search=native={}/build",
            destination.display()
        );
    };

    println!("cargo:rustc-link-lib=static=cxx-juce");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=c++");
        println!("cargo:rustc-link-lib=framework=Accelerate");
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=CoreAudio");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreMIDI");
        println!("cargo:rustc-link-lib=framework=IOKit");
    }

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=shell32");
        println!("cargo:rustc-link-lib=dylib=ole32");
    }

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=asound");
        println!("cargo:rustc-link-lib=jack");
    }
}
