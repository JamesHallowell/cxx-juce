//! Play and record from audio and MIDI I/O devices.

mod device;
mod device_callback;
mod device_manager;
mod device_type;

pub use device::{AudioDevice, AudioIODevice, BoxDynAudioDevice};
pub use device_callback::{AudioDeviceCallback, AudioIODeviceCallback, BoxDynAudioDeviceCallback};
pub use device_manager::{AudioDeviceManager, AudioDeviceSetup, ChannelCount};
pub use device_type::{AudioDeviceType, AudioIODeviceType, BoxDynAudioDeviceType};
pub use juce::SystemAudioVolume;

#[cxx::bridge(namespace = "juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        type JuceString = crate::juce_core::JuceString;
        type AudioSampleBuffer = crate::juce_audio_basics::AudioSampleBuffer;
        type AudioIODeviceCallback = crate::juce_audio_devices::AudioIODeviceCallback;
        type AudioIODeviceType = crate::juce_audio_devices::AudioIODeviceType;
        type AudioIODevice = crate::juce_audio_devices::AudioIODevice;
        /// Controls for the system volume.
        pub type SystemAudioVolume;

        #[cxx_name = "setMuted"]
        #[Self = "SystemAudioVolume"]
        /// Set the system audio output to be muted or unmuted.
        pub fn set_muted(muted: bool) -> bool;

        #[cxx_name = "isMuted"]
        #[Self = "SystemAudioVolume"]
        /// Returns true if the system audio output is muted.
        pub fn is_muted() -> bool;

        #[cxx_name = "setGain"]
        #[Self = "SystemAudioVolume"]
        /// Set the system volume.
        pub fn set_gain(gain: f32) -> bool;

        #[cxx_name = "getGain"]
        #[Self = "SystemAudioVolume"]
        /// Get the current system volume.
        pub fn get_gain() -> f32;
    }
}
