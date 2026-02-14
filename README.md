# cxx-juce 🧃

[![Build Status](https://github.com/JamesHallowell/cxx-juce/actions/workflows/ci.yml/badge.svg)](https://github.com/JamesHallowell/cxx-juce/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/cxx-juce.svg)](https://crates.io/crates/cxx-juce)
[![Docs.rs](https://docs.rs/cxx-juce/badge.svg)](https://docs.rs/cxx-juce)

**Rust bindings for [JUCE](https://juce.com/) using [cxx](https://github.com/dtolnay/cxx).**

## Overview

JUCE is something of an industry standard for audio applications, so it would be nice to be able to make use of it from
Rust.

Providing bindings for the entirety of JUCE would be a huge undertaking, and much of it would be duplicating
functionality already available to Rust in the standard library or via crates.io.

The goal for this crate is to provide bindings for a subset of JUCE, in particular the mature and thoroughly
battle-tested audio modules.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cxx-juce = "0.8"
```

## Dependencies

Refer to the [JUCE documentation](https://github.com/juce-framework/JUCE#building-juce-projects) for the dependencies
required to build JUCE on your platform.

## Crate Features

### `juce-8`

By default, this crate uses JUCE 7. To use JUCE 8 instead, enable the `juce-8` feature. Note that this changes the licensing terms for the JUCE modules, as described in the [License](#license) section below.

### `juce_audio_processors`

The `juce_audio_processors` module is not enabled by default, as it has additional dependencies and different licensing terms.

### `vst3`

Enables support for hosting VST3 plugins.

### `asio`

Enables the ASIO backend on Windows. To build with ASIO support:

1. Agree to Steinberg's licensing terms and download the ASIO SDK.
2. Enable the `asio` feature for this crate.
3. Set the `CXX_JUCE_ASIO_SDK_DIR` environment variable to the path of the extracted ASIO SDK.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license
  ([LICENSE-MIT](LICENSE-MIT))

at your option.

The licenses for the JUCE modules depend on the version of JUCE being used.

### JUCE 7 (default)

The `juce_core`, `juce_events`, `juce_audio_basics`, and `juce_audio_devices` modules are permissively licensed under the terms
of the [ISC license](https://www.isc.org/licenses/). The `juce_audio_processors` module is licensed under the terms of either the [GPL v3 license or the JUCE commercial license](https://github.com/juce-framework/JUCE/blob/7.0.12/LICENSE.md).

### JUCE 8 (enabled via the `juce-8` feature)

The `juce_core`, `juce_events`, `juce_audio_basics`, `juce_audio_devices`, and `juce_audio_processors` modules are licensed under the terms of either the [AGPL v3 license or the JUCE commercial license](https://github.com/juce-framework/JUCE/blob/8.0.12/LICENSE.md).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
