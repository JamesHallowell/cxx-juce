use std::{env, path::Path};

fn main() {
    let _ = cxx_build::bridge("src/lib.rs");

    let out_dir = env::var("OUT_DIR").unwrap();

    let mut cmake = cmake::Config::new("bridge");
    cmake.build_target("cxx-juce");

    cmake.define("CXX_JUCE_BINDINGS_DIR", format!("{out_dir}/cxxbridge"));

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

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=bridge/CMakelists.txt");
    println!("cargo:rerun-if-changed=bridge/cxx_juce.h");
    println!("cargo:rerun-if-changed=bridge/cxx_juce.cpp");
    println!("cargo:rerun-if-env-changed=CXX_JUCE_ASIO_SDK_DIR");

    println!(
        "cargo:rustc-link-search=native={}/build/cxx-juce_artefacts/{}",
        destination.display(),
        cmake.get_profile(),
    );
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
}
