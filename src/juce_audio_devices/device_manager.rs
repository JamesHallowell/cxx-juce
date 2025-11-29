use crate::{
    juce_audio_devices::{
        AudioDeviceCallback, AudioDeviceType, AudioIODeviceCallback, AudioIODeviceType,
    },
    juce_core::{BigInteger, JuceString},
    static_assert_offset_eq, static_assert_size_and_alignment, JuceError, JUCE,
};
use cxx::UniquePtr;
use slotmap::SlotMap;
use std::{
    ffi::{c_double, c_int},
    mem::ManuallyDrop,
    pin::Pin,
};

slotmap::new_key_type! {
    struct AudioCallbackKey;
}

/// A handle to a registered audio callback.
#[must_use]
pub struct AudioCallbackHandle {
    key: AudioCallbackKey,
}

/// Manages the state of an audio device.
pub struct AudioDeviceManager {
    device_manager: cxx::UniquePtr<juce::AudioDeviceManager>,
    callbacks: SlotMap<AudioCallbackKey, cxx::UniquePtr<AudioIODeviceCallback>>,
    _juce: JUCE,
}

impl AudioDeviceManager {
    /// Create a new [`AudioDeviceManager`].
    pub fn new(juce: &JUCE) -> Self {
        Self {
            device_manager: juce::make_audio_device_manager(),
            callbacks: SlotMap::with_key(),
            _juce: juce.clone(),
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
                &JuceString::empty(),
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

        let key = self.callbacks.insert(callback);

        AudioCallbackHandle { key }
    }

    /// Removes an audio callback.
    pub fn remove_audio_callback(&mut self, handle: AudioCallbackHandle) {
        if let Some(callback) = self.callbacks.remove(handle.key) {
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

/// The properties of an audio device.
#[repr(C)]
pub struct AudioDeviceSetup {
    output_device_name: ManuallyDrop<JuceString>,
    input_device_name: ManuallyDrop<JuceString>,
    sample_rate: c_double,
    buffer_size: c_int,
    input_channels: ManuallyDrop<BigInteger>,
    use_default_input_channels: bool,
    output_channels: ManuallyDrop<BigInteger>,
    use_default_output_channels: bool,
}

static_assert_size_and_alignment!(AudioDeviceSetup, juce::AudioDeviceSetupLayout);
static_assert_offset_eq!(
    AudioDeviceSetup,
    output_device_name,
    juce::AudioDeviceSetupLayout::OutputDeviceNameOffset
);
static_assert_offset_eq!(
    AudioDeviceSetup,
    input_device_name,
    juce::AudioDeviceSetupLayout::InputDeviceNameOffset
);
static_assert_offset_eq!(
    AudioDeviceSetup,
    sample_rate,
    juce::AudioDeviceSetupLayout::SampleRateOffset
);
static_assert_offset_eq!(
    AudioDeviceSetup,
    buffer_size,
    juce::AudioDeviceSetupLayout::BufferSizeOffset
);
static_assert_offset_eq!(
    AudioDeviceSetup,
    input_channels,
    juce::AudioDeviceSetupLayout::InputChannelsOffset
);
static_assert_offset_eq!(
    AudioDeviceSetup,
    use_default_input_channels,
    juce::AudioDeviceSetupLayout::UseDefaultInputChannelsOffset
);
static_assert_offset_eq!(
    AudioDeviceSetup,
    output_channels,
    juce::AudioDeviceSetupLayout::OutputChannelsOffset
);
static_assert_offset_eq!(
    AudioDeviceSetup,
    use_default_output_channels,
    juce::AudioDeviceSetupLayout::UseDefaultOutputChannelsOffset
);

impl Drop for AudioDeviceSetup {
    fn drop(&mut self) {
        juce::drop_audio_device_setup(self);
    }
}

unsafe impl cxx::ExternType for AudioDeviceSetup {
    type Id = cxx::type_id!("juce::AudioDeviceManager::AudioDeviceSetup");
    type Kind = cxx::kind::Trivial;
}

unsafe impl Send for AudioDeviceSetup {}

impl Default for AudioDeviceSetup {
    fn default() -> Self {
        juce::construct_audio_device_setup()
    }
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
    /// The name of the output device.
    pub fn output_device_name(&self) -> &str {
        self.output_device_name.as_ref()
    }

    /// Set the name of the output device.
    pub fn with_output_device_name(mut self, name: impl AsRef<str>) -> Self {
        self.output_device_name = ManuallyDrop::new(JuceString::new(name));
        self
    }

    /// The name of the input device.
    pub fn input_device_name(&self) -> &str {
        self.input_device_name.as_ref()
    }

    /// Set the name of the input device.
    pub fn with_input_device_name(mut self, name: impl AsRef<str>) -> Self {
        self.input_device_name = ManuallyDrop::new(JuceString::new(name));
        self
    }

    /// The sample rate in Hertz.
    pub fn sample_rate(&self) -> f64 {
        self.sample_rate
    }

    /// Set the sample rate in Hertz.
    pub fn with_sample_rate(mut self, sample_rate: f64) -> Self {
        self.sample_rate = sample_rate;
        self
    }

    /// The buffer size.
    pub fn buffer_size(&self) -> usize {
        self.buffer_size as usize
    }

    /// The buffer size to use.
    pub fn with_buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size as c_int;
        self
    }

    /// The number of input channels.
    pub fn input_channels(&self) -> ChannelCount {
        if self.use_default_input_channels {
            ChannelCount::Default
        } else {
            ChannelCount::Custom(self.input_channels.count_number_of_set_bits())
        }
    }

    // Set the number of input channels.
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

    /// The number of output channels.
    pub fn output_channels(&self) -> ChannelCount {
        if self.use_default_output_channels {
            ChannelCount::Default
        } else {
            ChannelCount::Custom(self.output_channels.count_number_of_set_bits())
        }
    }

    /// Set the number of output channels.
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

#[cxx::bridge(namespace = "cxx_juce")]
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

        #[namespace = "juce"]
        #[cxx_name = "String"]
        type JuceString = crate::juce_core::JuceString;

        #[namespace = "juce"]
        type AudioSampleBuffer = crate::juce_audio_basics::AudioSampleBuffer;

        #[namespace = "juce"]
        type AudioIODeviceCallback = crate::juce_audio_devices::AudioIODeviceCallback;

        #[namespace = "juce"]
        type AudioIODeviceType = crate::juce_audio_devices::AudioIODeviceType;

        #[namespace = "juce::AudioDeviceManager"]
        type AudioDeviceSetup = super::AudioDeviceSetup;

        #[cxx_name = "construct"]
        pub fn construct_audio_device_setup() -> AudioDeviceSetup;

        #[cxx_name = "drop"]
        pub fn drop_audio_device_setup(value: &mut AudioDeviceSetup);

        #[namespace = "juce"]
        pub type AudioDeviceManager;

        #[cxx_name = "makeUnique"]
        pub fn make_audio_device_manager() -> UniquePtr<AudioDeviceManager>;

        #[namespace = "juce"]
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

        #[namespace = "juce"]
        pub type AudioIODeviceTypeArray;

        pub fn size(self: &AudioIODeviceTypeArray) -> i32;

        #[rust_name = "get_unchecked"]
        pub fn getUnchecked(self: &AudioIODeviceTypeArray, index: i32) -> *mut AudioIODeviceType;

        #[namespace = "juce"]
        type AudioIODevice = crate::juce_audio_devices::AudioIODevice;
    }
}
