use crate::{define_trait, JuceString};
use cxx::UniquePtr;

pub use juce::{AudioIODevice, BoxDynAudioDevice};

#[cxx::bridge(namespace = "juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        pub type AudioIODevice;
        type BigInteger = crate::juce_core::BigInteger;
        type IntArray = crate::juce_core::IntArray;
        type DoubleArray = crate::juce_core::DoubleArray;
        type JuceString = crate::juce_core::JuceString;

        #[namespace = "cxx_juce"]
        type BoxDynAudioDevice = Box<dyn super::AudioDevice>;

        #[namespace = "cxx_juce"]
        #[cxx_name = "wrap"]
        fn wrap_audio_device(device: BoxDynAudioDevice) -> UniquePtr<AudioIODevice>;

        #[cxx_name = "getName"]
        pub fn get_name(self: &AudioIODevice) -> &JuceString;

        #[cxx_name = "getTypeName"]
        pub fn get_type_name(self: &AudioIODevice) -> &JuceString;

        #[cxx_name = "getCurrentSampleRate"]
        pub fn get_current_sample_rate(self: Pin<&mut AudioIODevice>) -> f64;

        #[cxx_name = "getCurrentBufferSizeSamples"]
        pub fn get_current_buffer_size_samples(self: Pin<&mut AudioIODevice>) -> i32;

        #[cxx_name = "getAvailableSampleRates"]
        pub fn get_available_sample_rates(self: Pin<&mut AudioIODevice>) -> DoubleArray;

        #[cxx_name = "getAvailableBufferSizes"]
        pub fn get_available_buffer_sizes(self: Pin<&mut AudioIODevice>) -> IntArray;

        #[cxx_name = "getActiveInputChannels"]
        pub fn get_active_input_channels(self: &AudioIODevice) -> BigInteger;

        #[cxx_name = "getActiveOutputChannels"]
        pub fn get_active_output_channels(self: &AudioIODevice) -> BigInteger;

        #[must_use]
        pub fn open(
            self: Pin<&mut AudioIODevice>,
            input_channels: &BigInteger,
            output_channels: &BigInteger,
            sample_rate: f64,
            buffer_size: i32,
        ) -> JuceString;

        pub fn close(self: Pin<&mut AudioIODevice>);
    }

    #[namespace = "cxx_juce"]
    extern "Rust" {
        type AudioDeviceImpl;

        #[Self = "AudioDeviceImpl"]
        unsafe fn drop(device: *mut BoxDynAudioDevice);

        #[Self = "AudioDeviceImpl"]
        fn name(device: &BoxDynAudioDevice) -> &str;

        #[Self = "AudioDeviceImpl"]
        fn type_name(device: &BoxDynAudioDevice) -> &str;

        #[Self = "AudioDeviceImpl"]
        fn sample_rate(device: &mut BoxDynAudioDevice) -> f64;

        #[Self = "AudioDeviceImpl"]
        fn buffer_size(device: &mut BoxDynAudioDevice) -> i32;

        #[Self = "AudioDeviceImpl"]
        fn available_sample_rates(device: &mut BoxDynAudioDevice) -> Vec<f64>;

        #[Self = "AudioDeviceImpl"]
        fn available_buffer_sizes(device: &mut BoxDynAudioDevice) -> Vec<i32>;

        #[Self = "AudioDeviceImpl"]
        fn open(device: &mut BoxDynAudioDevice, sample_rate: f64, buffer_size: i32) -> JuceString;

        #[Self = "AudioDeviceImpl"]
        fn close(device: &mut BoxDynAudioDevice);

        #[Self = "AudioDeviceImpl"]
        fn input_channels(device: &BoxDynAudioDevice) -> i32;

        #[Self = "AudioDeviceImpl"]
        fn output_channels(device: &BoxDynAudioDevice) -> i32;
    }
}

define_trait! {
    /// A trait representing an audio device.
    AudioDevice,
    AudioDeviceImpl,
    "cxx_juce::BoxDynAudioDevice",

    /// The name of the device.
    fn name(&self) -> &str;

    /// The type of the device.
    fn type_name(&self) -> &str;

    /// The current sample rate.
    fn sample_rate(&mut self) -> f64;

    /// The current buffer size.
    fn buffer_size(&mut self) -> i32;

    /// The available sample rates.
    fn available_sample_rates(&mut self) -> Vec<f64>;

    /// The available buffer sizes.
    fn available_buffer_sizes(&mut self) -> Vec<i32>;

    /// Tries to open the device so that it can be used for audio processing.
    fn open(&mut self, sample_rate: f64, buffer_size: i32) -> JuceString;

    /// Close the device.
    fn close(&mut self);

    /// The number of input channels.
    fn input_channels(&self) -> i32;

    /// The number of output channels.
    fn output_channels(&self) -> i32;
}

impl From<Box<dyn AudioDevice>> for UniquePtr<AudioIODevice> {
    fn from(device: Box<dyn AudioDevice>) -> Self {
        juce::wrap_audio_device(device)
    }
}
