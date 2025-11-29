use crate::juce_core::JuceString;
use cxx::UniquePtr;

pub use juce::{AudioIODevice, BoxDynAudioDevice};

#[cxx::bridge(namespace = "cxx_juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        #[namespace = "juce"]
        pub type AudioIODevice;

        type BoxDynAudioDevice = Box<dyn super::AudioDevice>;

        #[cxx_name = "wrapAudioDevice"]
        fn wrap_audio_device(device: BoxDynAudioDevice) -> UniquePtr<AudioIODevice>;

        #[namespace = "juce"]
        type BigInteger = crate::juce_core::BigInteger;

        #[namespace = "juce"]
        type IntArray = crate::juce_core::IntArray;

        #[namespace = "juce"]
        type DoubleArray = crate::juce_core::DoubleArray;

        #[namespace = "juce"]
        #[cxx_name = "String"]
        type JuceString = crate::juce_core::JuceString;

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

        #[rust_name = "close"]
        pub fn close(self: Pin<&mut AudioIODevice>);
    }

    #[namespace = "cxx_juce::BoxDynAudioIODeviceImpl"]
    extern "Rust" {
        unsafe fn drop(device: *mut BoxDynAudioDevice);

        fn name(device: &BoxDynAudioDevice) -> &str;

        fn type_name(device: &BoxDynAudioDevice) -> &str;

        fn sample_rate(device: &mut BoxDynAudioDevice) -> f64;

        fn buffer_size(device: &mut BoxDynAudioDevice) -> i32;

        fn available_sample_rates(device: &mut BoxDynAudioDevice) -> Vec<f64>;

        fn available_buffer_sizes(device: &mut BoxDynAudioDevice) -> Vec<i32>;

        fn open(device: &mut BoxDynAudioDevice, sample_rate: f64, buffer_size: i32) -> JuceString;

        fn close(device: &mut BoxDynAudioDevice);
    }
}

/// A trait representing an audio device.
pub trait AudioDevice {
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

unsafe impl cxx::ExternType for Box<dyn AudioDevice> {
    type Id = cxx::type_id!("cxx_juce::BoxDynAudioDevice");
    type Kind = cxx::kind::Trivial;
}

fn drop(device: *mut BoxDynAudioDevice) {
    unsafe { std::ptr::drop_in_place(device) };
}

fn name(device: &BoxDynAudioDevice) -> &str {
    device.name()
}

fn type_name(device: &BoxDynAudioDevice) -> &str {
    device.type_name()
}

fn sample_rate(device: &mut BoxDynAudioDevice) -> f64 {
    device.sample_rate()
}

fn buffer_size(device: &mut BoxDynAudioDevice) -> i32 {
    device.buffer_size()
}

fn available_sample_rates(device: &mut BoxDynAudioDevice) -> Vec<f64> {
    device.available_sample_rates()
}

fn available_buffer_sizes(device: &mut BoxDynAudioDevice) -> Vec<i32> {
    device.available_buffer_sizes()
}

fn open(device: &mut BoxDynAudioDevice, sample_rate: f64, buffer_size: i32) -> JuceString {
    device.open(sample_rate, buffer_size)
}

fn close(device: &mut BoxDynAudioDevice) {
    device.close()
}
