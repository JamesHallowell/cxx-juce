//! Rust bindings for [JUCE](https://juce.com/) using [cxx](https://github.com/dtolnay/cxx).

pub mod juce_audio_basics;
pub mod juce_audio_devices;
pub mod juce_audio_processors;
pub mod juce_core;
pub mod juce_events;
mod utils;

pub use {cxx::UniquePtr, juce_events::JUCE};

#[derive(Debug)]
pub struct JuceError(juce_core::JuceString);

impl std::fmt::Display for JuceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.0.as_ref())
    }
}

impl std::error::Error for JuceError {}
