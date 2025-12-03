use crate::{
    define_trait,
    juce_audio_devices::AudioIODevice,
    juce_core::{JuceString, StringArray},
};
use cxx::UniquePtr;
use std::pin::Pin;

pub use juce::{AudioIODeviceType, BoxDynAudioDeviceType};

impl AudioIODeviceType {
    pub fn get_input_device_names(&self) -> StringArray {
        self.get_device_names(true)
    }

    pub fn get_output_device_names(&self) -> StringArray {
        self.get_device_names(false)
    }

    pub fn create_device(
        self: Pin<&mut Self>,
        input_device_name: impl Into<JuceString>,
        output_device_name: impl Into<JuceString>,
    ) -> Option<UniquePtr<AudioIODevice>> {
        let input_device_name = input_device_name.into();
        let output_device_name = output_device_name.into();

        let device_ptr = self.create_device_raw(&input_device_name, &output_device_name);

        if device_ptr.is_null() {
            None
        } else {
            Some(unsafe { UniquePtr::from_raw(device_ptr) })
        }
    }
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        type AudioIODeviceType;
        type AudioIODevice = crate::juce_audio_devices::AudioIODevice;
        type StringArray = crate::juce_core::StringArray;
        type JuceString = crate::juce_core::JuceString;
        #[namespace = "cxx_juce"]
        type BoxDynAudioDevice = crate::juce_audio_devices::BoxDynAudioDevice;
        #[namespace = "cxx_juce"]
        type BoxDynAudioDeviceType = Box<dyn super::AudioDeviceType>;

        #[namespace = "cxx_juce"]
        #[cxx_name = "wrap"]
        fn wrap_audio_device_type(
            device_type: BoxDynAudioDeviceType,
        ) -> UniquePtr<AudioIODeviceType>;

        #[cxx_name = "getTypeName"]
        fn get_type_name(self: &AudioIODeviceType) -> &JuceString;

        #[rust_name = "scan_for_devices"]
        fn scanForDevices(self: Pin<&mut AudioIODeviceType>);

        #[cxx_name = "getDeviceNames"]
        fn get_device_names(self: &AudioIODeviceType, inputs: bool) -> StringArray;

        #[cxx_name = "createDevice"]
        fn create_device_raw(
            self: Pin<&mut AudioIODeviceType>,
            input_device_name: &JuceString,
            output_device_name: &JuceString,
        ) -> *mut AudioIODevice;
    }

    #[namespace = "cxx_juce"]
    extern "Rust" {
        type AudioDeviceTypeImpl;

        #[Self = "AudioDeviceTypeImpl"]
        unsafe fn drop(device: *mut BoxDynAudioDeviceType);

        #[Self = "AudioDeviceTypeImpl"]
        fn name(device_type: &BoxDynAudioDeviceType) -> String;

        #[Self = "AudioDeviceTypeImpl"]
        fn scan_for_devices(device_type: &mut BoxDynAudioDeviceType);

        #[Self = "AudioDeviceTypeImpl"]
        fn input_devices(device_type: &BoxDynAudioDeviceType) -> StringArray;

        #[Self = "AudioDeviceTypeImpl"]
        fn output_devices(device_type: &BoxDynAudioDeviceType) -> StringArray;

        #[Self = "AudioDeviceTypeImpl"]
        fn create_device(
            device_type: &mut BoxDynAudioDeviceType,
            input_device_name: &JuceString,
            output_device_name: &JuceString,
        ) -> UniquePtr<AudioIODevice>;

        #[Self = "AudioDeviceTypeImpl"]
        fn default_device_index(device_type: &BoxDynAudioDeviceType, for_input: bool) -> i32;

        #[Self = "AudioDeviceTypeImpl"]
        fn has_separate_inputs_and_outputs(device_type: &BoxDynAudioDeviceType) -> bool;
    }
}

define_trait! {
    /// A trait representing a type of audio driver (e.g. CoreAudio, ASIO, etc.).
    AudioDeviceType,
    AudioDeviceTypeImpl,
    "cxx_juce::BoxDynAudioDeviceType",

    /// The name of the type of driver.
    fn name(&self) -> String;

    /// Refreshes the drivers cached list of known devices.
    fn scan_for_devices(&mut self);

    /// Returns a list of known input devices.
    fn input_devices(&self) -> StringArray;

    /// Returns a list of the known output devices.
    fn output_devices(&self) -> StringArray;

    /// Create an [`AudioDevice`].
    fn create_device(
        &mut self,
        input_device_name: &JuceString,
        output_device_name: &JuceString,
    ) -> UniquePtr<AudioIODevice>;

    /// Returns the index of the default device.
    fn default_device_index(&self, for_input: bool) -> i32;

    /// Returns true if the device type has separate inputs and outputs.
    fn has_separate_inputs_and_outputs(&self) -> bool;
}

impl From<Box<dyn AudioDeviceType>> for UniquePtr<AudioIODeviceType> {
    fn from(audio_device: Box<dyn AudioDeviceType>) -> Self {
        juce::wrap_audio_device_type(audio_device)
    }
}
