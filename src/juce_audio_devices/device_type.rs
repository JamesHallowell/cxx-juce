use crate::{
    juce_audio_devices::AudioIODevice,
    juce_core::{JuceString, StringArray},
};
use cxx::UniquePtr;
use std::pin::Pin;

pub use juce::{AudioIODeviceType, BoxDynAudioIODeviceType};

impl AudioIODeviceType {
    pub fn get_input_device_names(&self) -> StringArray {
        self.get_device_names(true)
    }

    pub fn get_output_device_names(&self) -> StringArray {
        self.get_device_names(false)
    }

    pub fn create_device(
        self: Pin<&mut Self>,
        input_device_name: &str,
        output_device_name: &str,
    ) -> Option<UniquePtr<AudioIODevice>> {
        let input_device_name = JuceString::new(input_device_name);
        let output_device_name = JuceString::new(output_device_name);

        let device_ptr = self.create_device_raw(&input_device_name, &output_device_name);

        if device_ptr.is_null() {
            None
        } else {
            Some(unsafe { UniquePtr::from_raw(device_ptr) })
        }
    }
}

#[cxx::bridge(namespace = "cxx_juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        #[namespace = "juce"]
        type AudioIODeviceType;

        #[namespace = "juce"]
        type AudioIODevice = crate::juce_audio_devices::AudioIODevice;

        #[namespace = "juce"]
        type StringArray = crate::juce_core::StringArray;

        #[cxx_name = "wrapAudioDeviceType"]
        fn wrap_audio_device_type(
            device_type: BoxDynAudioIODeviceType,
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

        #[namespace = "juce"]
        #[cxx_name = "String"]
        type JuceString = crate::juce_core::JuceString;

        type BoxDynAudioDevice = crate::juce_audio_devices::BoxDynAudioDevice;

        type BoxDynAudioIODeviceType = Box<dyn super::AudioDeviceType>;
    }

    #[namespace = "cxx_juce::BoxDynAudioIODeviceTypeImpl"]
    extern "Rust" {
        unsafe fn drop(device: *mut BoxDynAudioIODeviceType);

        fn name(device_type: &BoxDynAudioIODeviceType) -> String;

        fn scan_for_devices(device_type: &mut BoxDynAudioIODeviceType);

        fn get_device_names(device_type: &BoxDynAudioIODeviceType, input: bool) -> Vec<String>;

        fn create_device(
            device_type: &mut BoxDynAudioIODeviceType,
            input_device_name: &JuceString,
            output_device_name: &JuceString,
        ) -> Result<UniquePtr<AudioIODevice>>;
    }
}

/// A trait representing a type of audio driver (e.g. CoreAudio, ASIO, etc.).
pub trait AudioDeviceType {
    /// The name of the type of driver.
    fn name(&self) -> String;

    /// Refreshes the drivers cached list of known devices.
    fn scan_for_devices(&mut self);

    /// Returns a list of known input devices.
    fn input_devices(&self) -> Vec<String>;

    /// Returns a list of the known output devices.
    fn output_devices(&self) -> Vec<String>;

    /// Create an [`AudioDevice`].
    fn create_device(
        &mut self,
        input_device_name: &str,
        output_device_name: &str,
    ) -> Option<UniquePtr<AudioIODevice>>;
}

unsafe impl cxx::ExternType for Box<dyn AudioDeviceType> {
    type Id = cxx::type_id!("cxx_juce::BoxDynAudioIODeviceType");
    type Kind = cxx::kind::Trivial;
}

impl From<Box<dyn AudioDeviceType>> for UniquePtr<AudioIODeviceType> {
    fn from(audio_device: Box<dyn AudioDeviceType>) -> Self {
        juce::wrap_audio_device_type(audio_device)
    }
}

fn drop(device_type: *mut BoxDynAudioIODeviceType) {
    unsafe { std::ptr::drop_in_place(device_type) };
}

fn name(device_type: &BoxDynAudioIODeviceType) -> String {
    device_type.name()
}

fn scan_for_devices(device_type: &mut BoxDynAudioIODeviceType) {
    device_type.scan_for_devices()
}

fn get_device_names(device_type: &BoxDynAudioIODeviceType, input: bool) -> Vec<String> {
    if input {
        device_type.input_devices()
    } else {
        device_type.output_devices()
    }
}

pub fn create_device(
    device_type: &mut BoxDynAudioIODeviceType,
    input_device_name: &JuceString,
    output_device_name: &JuceString,
) -> Result<UniquePtr<AudioIODevice>, String> {
    device_type
        .create_device(input_device_name.as_ref(), output_device_name.as_ref())
        .ok_or_else(|| "Could not create audio device".to_string())
}
