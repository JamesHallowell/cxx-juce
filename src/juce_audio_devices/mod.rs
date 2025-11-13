//! Play and record from audio and MIDI I/O devices.

mod device;
mod device_callback;
mod device_manager;
mod device_type;

pub use device::{AudioDevice, AudioIODevice, BoxDynAudioDevice};
pub use device_callback::{AudioDeviceCallback, AudioIODeviceCallback, BoxDynAudioDeviceCallback};
pub use device_manager::{AudioDeviceManager, AudioDeviceSetup, ChannelCount};
pub use device_type::{AudioDeviceType, AudioIODeviceType, BoxDynAudioIODeviceType};
pub use juce::SystemAudioVolume;

#[cxx::bridge(namespace = "cxx_juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        #[namespace = "juce"]
        #[cxx_name = "String"]
        type JuceString = crate::juce_core::JuceString;

        #[namespace = "juce"]
        type AudioSampleBuffer = crate::juce_audio_basics::AudioSampleBuffer;

        #[namespace = "juce"]
        type AudioIODeviceCallback = crate::juce_audio_devices::AudioIODeviceCallback;

        #[namespace = "juce"]
        type AudioIODeviceType = crate::juce_audio_devices::AudioIODeviceType;

        #[namespace = "juce"]
        type AudioIODevice = crate::juce_audio_devices::AudioIODevice;

        #[namespace = "juce"]
        /// Controls for the system volume.
        pub type SystemAudioVolume;

        #[namespace = "juce"]
        #[cxx_name = "setMuted"]
        #[Self = "SystemAudioVolume"]
        /// Set the system audio output to be muted or unmuted.
        pub fn set_muted(muted: bool) -> bool;

        #[namespace = "juce"]
        #[cxx_name = "isMuted"]
        #[Self = "SystemAudioVolume"]
        /// Returns true if the system audio output is muted.
        pub fn is_muted() -> bool;

        #[namespace = "juce"]
        #[cxx_name = "setGain"]
        #[Self = "SystemAudioVolume"]
        /// Set the system volume.
        pub fn set_gain(gain: f32) -> bool;

        #[namespace = "juce"]
        #[cxx_name = "getGain"]
        #[Self = "SystemAudioVolume"]
        /// Get the current system volume.
        pub fn get_gain() -> f32;
    }
}
