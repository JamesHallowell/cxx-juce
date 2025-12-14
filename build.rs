use std::{env, path::Path};

fn main() {
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    let bridges = [
        "src/juce_audio_basics/buffer.rs",
        "src/juce_audio_basics/filters.rs",
        "src/juce_audio_basics/midi.rs",
        "src/juce_audio_devices/device.rs",
        "src/juce_audio_devices/device_callback.rs",
        "src/juce_audio_devices/device_manager.rs",
        "src/juce_audio_devices/device_type.rs",
        "src/juce_audio_devices/midi_device_info.rs",
        "src/juce_audio_devices/midi_input.rs",
        "src/juce_audio_devices/midi_output.rs",
        "src/juce_audio_devices/mod.rs",
        "src/juce_audio_processors/plugin_description.rs",
        "src/juce_audio_processors/plugin_formats.rs",
        "src/juce_audio_processors/plugin_instance.rs",
        "src/juce_core/array.rs",
        "src/juce_core/bigint.rs",
        "src/juce_core/file.rs",
        "src/juce_core/memory.rs",
        "src/juce_core/string.rs",
        "src/juce_core/system.rs",
        "src/juce_core/time.rs",
        "src/juce_events/application.rs",
        "src/juce_events/message_manager.rs",
        "src/juce_events/mod.rs",
    ];
    for bridge in bridges.iter() {
        let _ = cxx_build::bridge(bridge);
        println!("cargo:rerun-if-changed={bridge}");
    }

    let mut cmake = cmake::Config::new("bridge");
    cmake.build_target("cxx-juce");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    cmake.define(
        "CXX_JUCE_BRIDGE_FILES",
        bridges
            .iter()
            .map(|bridge| format!("{}/target/cxxbridge/cxx-juce/{}.cc", manifest_dir, bridge))
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

    if cfg!(target_os = "macos") {
        cmake.define("CMAKE_OSX_DEPLOYMENT_TARGET", "12.0");
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
        println!("cargo:rustc-link-lib=framework=AudioUnit");
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=CoreAudio");
        println!("cargo:rustc-link-lib=framework=CoreAudioKit");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreMIDI");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=QuartzCore");
    }

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=shell32");
        println!("cargo:rustc-link-lib=dylib=ole32");
        println!("cargo:rustc-link-lib=dylib=gdi32");
        println!("cargo:rustc-link-lib=dylib=oleaut32");
        println!("cargo:rustc-link-lib=dylib=comdlg32");
    }

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=asound");
        println!("cargo:rustc-link-lib=jack");
    }
}
