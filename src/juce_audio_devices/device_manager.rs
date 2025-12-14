use crate::{
    define_juce_type,
    juce_audio_devices::{
        AudioDeviceCallback, AudioDeviceType, AudioIODeviceCallback, AudioIODeviceType,
    },
    juce_core::{BigInteger, JuceString},
    JuceError, JUCE,
};
use cxx::UniquePtr;
use std::{
    collections::HashMap,
    ffi::{c_double, c_int},
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
};

/// A handle to a registered audio callback.
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct AudioCallbackHandle {
    key: u64,
}

impl AudioCallbackHandle {
    fn get() -> AudioCallbackHandle {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let next = NEXT.fetch_add(1, Ordering::Relaxed);
        AudioCallbackHandle { key: next }
    }
}

/// Manages the state of an audio device.
pub struct AudioDeviceManager {
    device_manager: UniquePtr<juce::AudioDeviceManager>,
    callbacks: HashMap<AudioCallbackHandle, UniquePtr<AudioIODeviceCallback>>,
}

impl AudioDeviceManager {
    /// Create a new [`AudioDeviceManager`].
    pub fn new(_: &JUCE) -> Self {
        Self {
            device_manager: juce::make_audio_device_manager(),
            callbacks: HashMap::default(),
        }
    }

    /// Resets to a default device setup.
    pub fn initialise(
        &mut self,
        input_channels: i32,
        output_channels: i32,
    ) -> Result<(), JuceError> {
        let result = unsafe {
            self.device_manager.pin_mut().initialise(
                input_channels,
                output_channels,
                std::ptr::null(),
                false,
                &JuceString::default(),
                std::ptr::null(),
            )
        };

        if result.is_empty() {
            Ok(())
        } else {
            Err(JuceError(result))
        }
    }

    /// Get the current device setup.
    pub fn audio_device_setup(&self) -> AudioDeviceSetup {
        self.device_manager.get_audio_device_setup()
    }

    /// Changes the current device or its settings.
    pub fn set_audio_device_setup(
        &mut self,
        setup: &AudioDeviceSetup,
        treat_as_chosen_device: bool,
    ) -> Result<(), JuceError> {
        let result = self
            .device_manager
            .pin_mut()
            .set_audio_device_setup(setup, treat_as_chosen_device);

        if result.is_empty() {
            Ok(())
        } else {
            Err(JuceError(result))
        }
    }

    /// Play a test sound.
    pub fn play_test_sound(&mut self) {
        self.device_manager.pin_mut().play_test_sound();
    }

    /// Get the available device types.
    pub fn device_types(&mut self) -> Vec<Pin<&mut AudioIODeviceType>> {
        let available_device_types = self.device_manager.pin_mut().get_available_device_types();

        (0..available_device_types.size())
            .map(|i| available_device_types.get_unchecked(i))
            .map(|device_type_ptr| {
                let device_type_ref = unsafe {
                    device_type_ptr
                        .as_mut()
                        .expect("device type ptr should not be null")
                };

                unsafe { Pin::new_unchecked(device_type_ref) }
            })
            .collect()
    }

    /// Get the current device type.
    pub fn current_device_type(&mut self) -> Option<Pin<&mut AudioIODeviceType>> {
        let device_type = self.device_manager.get_current_device_type_object();
        unsafe { device_type.as_mut().map(|ptr| Pin::new_unchecked(ptr)) }
    }

    /// Get the current [`AudioDevice`].
    pub fn current_device(&mut self) -> Option<Pin<&mut juce::AudioIODevice>> {
        let current_device = self.device_manager.pin_mut().get_current_audio_device();

        unsafe { current_device.as_mut().map(|ptr| Pin::new_unchecked(ptr)) }
    }

    /// Registers an audio callback.
    pub fn add_audio_callback(
        &mut self,
        callback: impl AudioDeviceCallback + 'static,
    ) -> AudioCallbackHandle {
        let callback: Box<dyn AudioDeviceCallback> = Box::new(callback);
        let callback: UniquePtr<_> = callback.into();

        unsafe {
            self.device_manager
                .pin_mut()
                .add_audio_callback(callback.as_mut_ptr());
        }

        let handle = AudioCallbackHandle::get();
        self.callbacks.insert(handle, callback);
        handle
    }

    /// Removes an audio callback.
    pub fn remove_audio_callback(&mut self, handle: AudioCallbackHandle) {
        if let Some(callback) = self.callbacks.remove(&handle) {
            unsafe {
                self.device_manager
                    .pin_mut()
                    .remove_audio_callback(callback.as_mut_ptr());
            }
        }
    }

    /// Registers an audio device type.
    pub fn add_audio_device_type(&mut self, device_type: impl AudioDeviceType + 'static) {
        let device_type: Box<dyn AudioDeviceType> = Box::new(device_type);

        self.device_manager
            .pin_mut()
            .add_audio_device_type(device_type.into());
    }

    /// Set the current audio device type to use.
    pub fn set_current_audio_device_type(
        &mut self,
        device_type: &str,
        treat_as_chosen_device: bool,
    ) {
        self.device_manager
            .pin_mut()
            .set_current_audio_device_type(&JuceString::new(device_type), treat_as_chosen_device)
    }
}

define_juce_type! {
    /// The properties of an audio device.
    AudioDeviceSetup,
    fields = {
        pub output_device_name: JuceString = {
            offset = juce::AudioDeviceSetupLayout::OutputDeviceNameOffset,
            with = with_output_device_name,
        },
        pub input_device_name: JuceString = {
            offset = juce::AudioDeviceSetupLayout::InputDeviceNameOffset,
            with = with_input_device_name,
        },
        pub sample_rate: c_double = {
            offset = juce::AudioDeviceSetupLayout::SampleRateOffset,
            with = with_sample_rate,
        },
        pub buffer_size: c_int = {
            offset = juce::AudioDeviceSetupLayout::BufferSizeOffset,
            with = with_buffer_size,
        },
        pub input_channels: BigInteger = {
            offset = juce::AudioDeviceSetupLayout::InputChannelsOffset,
        },
        pub use_default_input_channels: bool = {
            offset = juce::AudioDeviceSetupLayout::UseDefaultInputChannelsOffset,
        },
        pub output_channels: BigInteger = {
            offset = juce::AudioDeviceSetupLayout::OutputChannelsOffset,
        },
        pub use_default_output_channels: bool = {
            offset = juce::AudioDeviceSetupLayout::UseDefaultOutputChannelsOffset,
        },
    },
    layout = juce::AudioDeviceSetupLayout,
    cxx_name = "juce::AudioDeviceSetup",
    default = juce::audio_device_setup_new,
}

/// The number of channels to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelCount {
    /// Use the default number of channels for the device.
    Default,

    /// Use a custom number of channels.
    Custom(i32),
}

unsafe impl Send for ChannelCount {}

impl AudioDeviceSetup {
    pub fn input_channels(&self) -> ChannelCount {
        if self.use_default_input_channels {
            ChannelCount::Default
        } else {
            ChannelCount::Custom(self.input_channels.count_number_of_set_bits())
        }
    }

    pub fn with_input_channels(mut self, channels: ChannelCount) -> Self {
        match channels {
            ChannelCount::Default => {
                self.use_default_input_channels = true;
            }
            ChannelCount::Custom(count) => {
                self.use_default_input_channels = false;
                self.input_channels.clear().set_range(0, count, true);
            }
        }

        self
    }

    pub fn output_channels(&self) -> ChannelCount {
        if self.use_default_output_channels {
            ChannelCount::Default
        } else {
            ChannelCount::Custom(self.output_channels.count_number_of_set_bits())
        }
    }

    pub fn with_output_channels(mut self, channels: ChannelCount) -> Self {
        match channels {
            ChannelCount::Default => {
                self.use_default_output_channels = true;
            }
            ChannelCount::Custom(count) => {
                self.use_default_output_channels = false;
                self.output_channels.clear().set_range(0, count, true);
            }
        }

        self
    }
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum AudioDeviceSetupLayout {
        Size = 128,
        Alignment = 8,

        OutputDeviceNameOffset = 0,
        InputDeviceNameOffset = 8,
        SampleRateOffset = 16,
        BufferSizeOffset = 24,
        InputChannelsOffset = 32,
        UseDefaultInputChannelsOffset = 72,
        OutputChannelsOffset = 80,
        UseDefaultOutputChannelsOffset = 120,
    }

    unsafe extern "C++" {
        include!("cxx_juce.h");

        type JuceString = crate::juce_core::JuceString;
        type AudioSampleBuffer = crate::juce_audio_basics::AudioSampleBuffer;
        type AudioIODeviceCallback = crate::juce_audio_devices::AudioIODeviceCallback;
        type AudioIODeviceType = crate::juce_audio_devices::AudioIODeviceType;
        type AudioDeviceSetup = super::AudioDeviceSetup;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        pub fn audio_device_setup_new() -> AudioDeviceSetup;

        pub type AudioDeviceManager;

        #[namespace = "cxx_juce"]
        #[cxx_name = "makeUnique"]
        pub fn make_audio_device_manager() -> UniquePtr<AudioDeviceManager>;

        type XmlElement;

        #[must_use]
        unsafe fn initialise(
            self: Pin<&mut AudioDeviceManager>,
            num_input_channels: i32,
            num_output_channels: i32,
            saved_state: *const XmlElement,
            select_default_device_on_failure: bool,
            preferred_default_device_name: &JuceString,
            preferred_setup_options: *const AudioDeviceSetup,
        ) -> JuceString;

        #[rust_name = "get_audio_device_setup"]
        pub fn getAudioDeviceSetup(self: &AudioDeviceManager) -> AudioDeviceSetup;

        #[cxx_name = "setAudioDeviceSetup"]
        #[must_use]
        pub fn set_audio_device_setup(
            self: Pin<&mut AudioDeviceManager>,
            setup: &AudioDeviceSetup,
            treat_as_chosen_device: bool,
        ) -> JuceString;

        #[cxx_name = "getCurrentAudioDevice"]
        fn get_current_audio_device(self: &AudioDeviceManager) -> *mut AudioIODevice;

        #[rust_name = "get_available_device_types"]
        pub fn getAvailableDeviceTypes(
            self: Pin<&mut AudioDeviceManager>,
        ) -> &AudioIODeviceTypeArray;

        #[rust_name = "get_current_device_type_object"]
        pub fn getCurrentDeviceTypeObject(self: &AudioDeviceManager) -> *mut AudioIODeviceType;

        #[rust_name = "play_test_sound"]
        pub fn playTestSound(self: Pin<&mut AudioDeviceManager>);

        #[cxx_name = "addAudioCallback"]
        unsafe fn add_audio_callback(
            self: Pin<&mut AudioDeviceManager>,
            callback: *mut AudioIODeviceCallback,
        );

        #[cxx_name = "removeAudioCallback"]
        unsafe fn remove_audio_callback(
            self: Pin<&mut AudioDeviceManager>,
            callback: *mut AudioIODeviceCallback,
        );

        #[cxx_name = "addAudioDeviceType"]
        pub fn add_audio_device_type(
            self: Pin<&mut AudioDeviceManager>,
            device_type: UniquePtr<AudioIODeviceType>,
        );

        #[cxx_name = "setCurrentAudioDeviceType"]
        pub fn set_current_audio_device_type(
            self: Pin<&mut AudioDeviceManager>,
            device_type: &JuceString,
            treat_as_chosen_device: bool,
        );

        pub type AudioIODeviceTypeArray;

        pub fn size(self: &AudioIODeviceTypeArray) -> i32;

        #[rust_name = "get_unchecked"]
        pub fn getUnchecked(self: &AudioIODeviceTypeArray, index: i32) -> *mut AudioIODeviceType;

        type AudioIODevice = crate::juce_audio_devices::AudioIODevice;
    }
}
