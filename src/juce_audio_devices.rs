//! Play and record from audio and MIDI I/O devices.

use {
    crate::{juce, Result, JUCE},
    slotmap::SlotMap,
    std::{
        ops::{Index, IndexMut},
        pin::Pin,
    },
};

/// A multi-channel buffer of read-only audio samples.
pub struct InputAudioSampleBuffer<'a> {
    buffer: &'a juce::AudioSampleBuffer,
}

impl<'a> InputAudioSampleBuffer<'a> {
    pub(crate) fn new(buffer: &'a juce::AudioSampleBuffer) -> Self {
        Self { buffer }
    }

    /// Returns the numbers of channels in the buffer.
    pub fn channels(&self) -> usize {
        self.buffer.get_num_channels() as usize
    }

    /// Returns the number of samples for each channel.
    pub fn samples(&self) -> usize {
        self.buffer.get_num_samples() as usize
    }
}

impl Index<usize> for InputAudioSampleBuffer<'_> {
    type Output = [f32];

    fn index(&self, channel: usize) -> &Self::Output {
        if self.channels() < channel {
            panic!("channel out of bounds");
        }

        let ptr = self.buffer.get_read_pointer(channel as i32);
        let len = self.samples();

        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}

/// A multi-channel buffer of read-write audio samples.
pub struct OutputAudioSampleBuffer<'a> {
    buffer: Pin<&'a mut juce::AudioSampleBuffer>,
}

impl<'a> OutputAudioSampleBuffer<'a> {
    pub(crate) fn new(buffer: Pin<&'a mut juce::AudioSampleBuffer>) -> Self {
        Self { buffer }
    }

    /// Returns the numbers of channels in the buffer.
    pub fn channels(&self) -> usize {
        self.buffer.get_num_channels() as usize
    }

    /// Returns the number of samples for each channel.
    pub fn samples(&self) -> usize {
        self.buffer.get_num_samples() as usize
    }

    /// Clear all the samples for all the channels.
    pub fn clear(&mut self) {
        self.buffer.as_mut().clear();
    }
}

impl Index<usize> for OutputAudioSampleBuffer<'_> {
    type Output = [f32];

    fn index(&self, channel: usize) -> &Self::Output {
        if self.channels() < channel {
            panic!("channel out of bounds");
        }

        let ptr = self.buffer.get_read_pointer(channel as i32);
        let len = self.samples();

        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}

impl IndexMut<usize> for OutputAudioSampleBuffer<'_> {
    fn index_mut(&mut self, channel: usize) -> &mut Self::Output {
        if self.channels() < channel {
            panic!("channel out of bounds");
        }

        let ptr = self.buffer.as_mut().get_write_pointer(channel as i32);
        let len = self.samples();

        unsafe { std::slice::from_raw_parts_mut(ptr, len) }
    }
}

/// The properties of an audio device.
pub struct AudioDeviceSetup(cxx::UniquePtr<juce::AudioDeviceSetup>);

unsafe impl Send for AudioDeviceSetup {}

impl Default for AudioDeviceSetup {
    fn default() -> Self {
        Self(juce::create_audio_device_setup())
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
        self.0.output_device_name()
    }

    /// Set the name of the output device.
    pub fn with_output_device_name(mut self, name: impl AsRef<str>) -> Self {
        self.0.pin_mut().set_output_device_name(name.as_ref());
        self
    }

    /// The name of the input device.
    pub fn input_device_name(&self) -> &str {
        self.0.input_device_name()
    }

    /// Set the name of the input device.
    pub fn with_input_device_name(mut self, name: impl AsRef<str>) -> Self {
        self.0.pin_mut().set_input_device_name(name.as_ref());
        self
    }

    /// The sample rate in Hertz.
    pub fn sample_rate(&self) -> f64 {
        self.0.sample_rate()
    }

    /// Set the sample rate in Hertz.
    pub fn with_sample_rate(mut self, sample_rate: f64) -> Self {
        self.0.pin_mut().set_sample_rate(sample_rate);
        self
    }

    /// The buffer size.
    pub fn buffer_size(&self) -> usize {
        self.0.buffer_size() as usize
    }

    /// The buffer size to use.
    pub fn with_buffer_size(mut self, buffer_size: usize) -> Self {
        self.0.pin_mut().set_buffer_size(buffer_size as i32);
        self
    }

    /// The number of input channels.
    pub fn input_channels(&self) -> ChannelCount {
        if self.0.using_default_input_channels() {
            ChannelCount::Default
        } else {
            ChannelCount::Custom(self.0.number_of_input_channels())
        }
    }

    // Set the number of input channels.
    pub fn with_input_channels(mut self, channels: ChannelCount) -> Self {
        match channels {
            ChannelCount::Default => {
                self.0.pin_mut().use_default_input_channels(true);
            }
            ChannelCount::Custom(count) => {
                self.0.pin_mut().use_default_input_channels(false);
                self.0.pin_mut().set_number_of_input_channels(count);
            }
        }

        self
    }

    /// The number of output channels.
    pub fn output_channels(&self) -> ChannelCount {
        if self.0.using_default_output_channels() {
            ChannelCount::Default
        } else {
            ChannelCount::Custom(self.0.number_of_output_channels())
        }
    }

    /// Set the number of output channels.
    pub fn with_output_channels(mut self, channels: ChannelCount) -> Self {
        match channels {
            ChannelCount::Default => {
                self.0.pin_mut().use_default_output_channels(true);
            }
            ChannelCount::Custom(count) => {
                self.0.pin_mut().use_default_output_channels(false);
                self.0.pin_mut().set_number_of_output_channels(count);
            }
        }

        self
    }
}

slotmap::new_key_type! {
    struct AudioCallbackKey;
}

/// Manages the state of an audio device.
pub struct AudioDeviceManager {
    device_manager: cxx::UniquePtr<juce::AudioDeviceManager>,
    callbacks: SlotMap<AudioCallbackKey, cxx::UniquePtr<juce::AudioCallbackWrapper>>,
    _juce: JUCE,
}

impl AudioDeviceManager {
    /// Create a new [`AudioDeviceManager`].
    pub fn new(juce: &JUCE) -> Self {
        Self {
            device_manager: juce::create_audio_device_manager(),
            callbacks: SlotMap::with_key(),
            _juce: juce.clone(),
        }
    }

    /// Resets to a default device setup.
    pub fn initialise(&mut self, input_channels: usize, output_channels: usize) -> Result<()> {
        self.device_manager
            .pin_mut()
            .initialise_with_default_devices(input_channels as i32, output_channels as i32)
    }

    /// Get the current device setup.
    pub fn audio_device_setup(&self) -> AudioDeviceSetup {
        AudioDeviceSetup(self.device_manager.get_audio_device_setup())
    }

    /// Changes the current device or its settings.
    pub fn set_audio_device_setup(&mut self, setup: &AudioDeviceSetup) {
        self.device_manager
            .pin_mut()
            .set_audio_device_setup(&setup.0);
    }

    /// Play a test sound.
    pub fn play_test_sound(&mut self) {
        self.device_manager.pin_mut().play_test_sound();
    }

    /// Get the available device types.
    pub fn device_types(&mut self) -> Vec<impl AudioIODeviceType + '_> {
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
    pub fn current_device_type(&self) -> Option<impl AudioIODeviceType + '_> {
        let device_type = self.device_manager.get_current_device_type_object();

        unsafe { device_type.as_mut().map(|ptr| Pin::new_unchecked(ptr)) }
    }

    /// Get the current [`AudioIODevice`].
    pub fn current_device(&mut self) -> Option<impl AudioIODevice + '_> {
        let current_device = self.device_manager.pin_mut().get_current_audio_device();

        unsafe { current_device.as_mut().map(|ptr| Pin::new_unchecked(ptr)) }
    }

    /// Registers an audio callback.
    pub fn add_audio_callback(
        &mut self,
        callback: impl AudioIODeviceCallback + 'static,
    ) -> AudioCallbackHandle {
        let callback = Box::new(callback);
        let callback = juce::wrap_audio_callback(Box::new(callback));

        self.device_manager.pin_mut().add_audio_callback(&callback);
        let key = self.callbacks.insert(callback);

        AudioCallbackHandle { key }
    }

    /// Removes an audio callback.
    pub fn remove_audio_callback(&mut self, handle: AudioCallbackHandle) {
        if let Some(callback) = self.callbacks.remove(handle.key) {
            self.device_manager
                .pin_mut()
                .remove_audio_callback(&callback);
        }
    }

    /// Registers an audio device type.
    pub fn add_audio_device_type(&mut self, device_type: impl AudioIODeviceType + 'static) {
        let device_type = Box::new(device_type);
        self.device_manager
            .pin_mut()
            .add_audio_device_type(Box::new(device_type));
    }

    /// Set the current audio device type to use.
    pub fn set_current_audio_device_type(&mut self, device_type: &str) {
        self.device_manager
            .pin_mut()
            .set_current_audio_device_type(device_type);
    }
}

/// A trait that can be implemented to receive audio callbacks.
///
/// Types that implement this trait can be registered with [`AudioDeviceManager::add_audio_callback`].
///
/// This trait requires that implementors are [`Send`] because the callbacks will occur on the audio thread.
pub trait AudioIODeviceCallback: Send {
    /// Called when the audio device is about to start.
    fn about_to_start(&mut self, device: &mut dyn AudioIODevice);

    /// Process a block of incoming and outgoing audio.
    fn process_block(
        &mut self,
        input: &InputAudioSampleBuffer<'_>,
        output: &mut OutputAudioSampleBuffer<'_>,
    );

    /// Called when the audio device has stopped.
    fn stopped(&mut self);
}

pub(crate) type BoxedAudioIODeviceCallback = Box<dyn AudioIODeviceCallback>;
pub(crate) type BoxedAudioIODeviceType = Box<dyn AudioIODeviceType>;
pub(crate) type BoxedAudioIODevice = Box<dyn AudioIODevice>;

/// A handle to a registered audio callback.
#[must_use]
pub struct AudioCallbackHandle {
    key: AudioCallbackKey,
}

/// A trait representing a type of audio driver (e.g. CoreAudio, ASIO, etc.).
pub trait AudioIODeviceType {
    /// The name of the type of driver.
    fn name(&self) -> String;

    /// Refreshes the drivers cached list of known devices.
    fn scan_for_devices(&mut self);

    /// Returns a list of known input devices.
    fn input_devices(&self) -> Vec<String>;

    /// Returns a list of the known output devices.
    fn output_devices(&self) -> Vec<String>;

    /// Create an [`AudioIODevice`].
    fn create_device(
        &mut self,
        input_device_name: &str,
        output_device_name: &str,
    ) -> Option<Box<dyn AudioIODevice>>;
}

impl AudioIODeviceType for Pin<&mut juce::AudioIODeviceType> {
    fn name(&self) -> String {
        juce::get_type_name(self)
    }

    fn scan_for_devices(&mut self) {
        juce::AudioIODeviceType::scan_for_devices(self.as_mut())
    }

    fn input_devices(&self) -> Vec<String> {
        juce::get_input_device_names(self)
    }

    fn output_devices(&self) -> Vec<String> {
        juce::get_output_device_names(self)
    }

    fn create_device(
        &mut self,
        input_device_name: &str,
        output_device_name: &str,
    ) -> Option<Box<dyn AudioIODevice>> {
        let device = juce::new_device(self.as_mut(), input_device_name, output_device_name);

        (!device.is_null()).then(|| -> Box<dyn AudioIODevice> { Box::new(device) })
    }
}

/// A trait representing an audio device.
pub trait AudioIODevice {
    /// The name of the device.
    fn name(&self) -> &str;

    /// The type of the device.
    fn type_name(&self) -> &str;

    /// The current sample rate.
    fn sample_rate(&mut self) -> f64;

    /// The current buffer size.
    fn buffer_size(&mut self) -> usize;

    /// The available sample rates.
    fn available_sample_rates(&mut self) -> Vec<f64>;

    /// The available buffer sizes.
    fn available_buffer_sizes(&mut self) -> Vec<usize>;

    /// Tries to open the device so that it can be used for audio processing.
    fn open(&mut self, sample_rate: f64, buffer_size: usize) -> Result<()>;

    /// Close the device.
    fn close(&mut self);

    /// The number of input channels.
    fn input_channels(&self) -> i32;

    /// The number of output channels.
    fn output_channels(&self) -> i32;
}

impl AudioIODevice for Pin<&mut juce::AudioIODevice> {
    fn name(&self) -> &str {
        juce::get_device_name(self)
    }

    fn type_name(&self) -> &str {
        juce::get_device_type_name(self)
    }

    fn sample_rate(&mut self) -> f64 {
        juce::AudioIODevice::get_current_sample_rate(self.as_mut())
    }

    fn buffer_size(&mut self) -> usize {
        juce::AudioIODevice::get_current_buffer_size_samples(self.as_mut()) as usize
    }

    fn available_sample_rates(&mut self) -> Vec<f64> {
        juce::get_available_sample_rates(self.as_mut())
    }

    fn available_buffer_sizes(&mut self) -> Vec<usize> {
        juce::get_available_buffer_sizes(self.as_mut())
    }

    fn open(&mut self, sample_rate: f64, buffer_size: usize) -> Result<()> {
        juce::open(self.as_mut(), sample_rate, buffer_size)
    }

    fn close(&mut self) {
        juce::AudioIODevice::close(self.as_mut());
    }

    fn input_channels(&self) -> i32 {
        juce::count_active_input_channels(self)
    }

    fn output_channels(&self) -> i32 {
        juce::count_active_output_channels(self)
    }
}

impl AudioIODevice for cxx::UniquePtr<juce::AudioIODevice> {
    fn name(&self) -> &str {
        self.as_ref().map(juce::get_device_name).unwrap_or_default()
    }

    fn type_name(&self) -> &str {
        self.as_ref()
            .map(juce::get_device_type_name)
            .unwrap_or_default()
    }

    fn sample_rate(&mut self) -> f64 {
        self.as_mut()
            .map(|this| this.get_current_sample_rate())
            .unwrap_or_default()
    }

    fn buffer_size(&mut self) -> usize {
        self.as_mut()
            .map(|this| this.get_current_buffer_size_samples() as usize)
            .unwrap_or_default()
    }

    fn available_sample_rates(&mut self) -> Vec<f64> {
        self.as_mut()
            .map(juce::get_available_sample_rates)
            .unwrap_or_default()
    }

    fn available_buffer_sizes(&mut self) -> Vec<usize> {
        self.as_mut()
            .map(juce::get_available_buffer_sizes)
            .unwrap_or_default()
    }

    fn open(&mut self, sample_rate: f64, buffer_size: usize) -> Result<()> {
        if let Some(this) = self.as_mut() {
            juce::open(this, sample_rate, buffer_size)?;
        }

        Ok(())
    }

    fn close(&mut self) {
        if let Some(this) = self.as_mut() {
            this.close();
        }
    }

    fn input_channels(&self) -> i32 {
        self.as_ref()
            .map(juce::count_active_input_channels)
            .unwrap_or_default()
    }

    fn output_channels(&self) -> i32 {
        self.as_ref()
            .map(juce::count_active_output_channels)
            .unwrap_or_default()
    }
}

pub(crate) mod ffi {
    use super::*;

    pub mod audio_io_device_callback {
        use super::*;

        pub fn about_to_start(
            mut self_: Pin<&mut BoxedAudioIODeviceCallback>,
            mut device: Pin<&mut juce::AudioIODevice>,
        ) {
            self_.about_to_start(&mut device.as_mut());
        }

        pub fn process_block(
            mut self_: Pin<&mut BoxedAudioIODeviceCallback>,
            input: &juce::AudioSampleBuffer,
            output: Pin<&mut juce::AudioSampleBuffer>,
        ) {
            let input = InputAudioSampleBuffer::new(input);
            let mut output = OutputAudioSampleBuffer::new(output);

            self_.process_block(&input, &mut output);
        }

        pub fn stopped(mut self_: Pin<&mut BoxedAudioIODeviceCallback>) {
            self_.stopped()
        }
    }

    pub mod audio_io_device_type {
        use {super::*, std::ptr::null_mut};

        pub fn name(self_: &BoxedAudioIODeviceType) -> String {
            self_.name()
        }

        pub fn scan_for_devices(mut self_: Pin<&mut BoxedAudioIODeviceType>) {
            self_.scan_for_devices()
        }

        pub fn get_device_names(self_: &BoxedAudioIODeviceType, input: bool) -> Vec<String> {
            if input {
                self_.input_devices()
            } else {
                self_.output_devices()
            }
        }

        pub fn create_device(
            mut self_: Pin<&mut BoxedAudioIODeviceType>,
            input_name: &str,
            output_name: &str,
        ) -> *mut BoxedAudioIODevice {
            let device = self_.as_mut().create_device(input_name, output_name);

            device
                .map(|device| Box::into_raw(Box::new(device)))
                .unwrap_or(null_mut())
        }

        pub fn destroy_device(device: *mut BoxedAudioIODevice) {
            if device.is_null() {
                return;
            }

            let _ = unsafe { Box::from_raw(device) };
        }
    }

    pub mod audio_io_device {
        use super::*;

        pub fn device_name(self_: &BoxedAudioIODevice) -> String {
            self_.name().to_string()
        }

        pub fn device_type_name(self_: &BoxedAudioIODevice) -> String {
            self_.type_name().to_string()
        }

        pub fn device_sample_rate(mut self_: Pin<&mut BoxedAudioIODevice>) -> f64 {
            self_.sample_rate()
        }

        pub fn device_buffer_size(mut self_: Pin<&mut BoxedAudioIODevice>) -> usize {
            self_.buffer_size()
        }

        pub fn device_available_sample_rates(mut self_: Pin<&mut BoxedAudioIODevice>) -> Vec<f64> {
            self_.available_sample_rates()
        }

        pub fn device_available_buffer_sizes(
            mut self_: Pin<&mut BoxedAudioIODevice>,
        ) -> Vec<usize> {
            self_.available_buffer_sizes()
        }

        pub fn device_open(
            mut self_: Pin<&mut BoxedAudioIODevice>,
            sample_rate: f64,
            buffer_size: usize,
        ) -> String {
            match self_.open(sample_rate, buffer_size) {
                Ok(()) => String::default(),
                Err(error) => error.to_string(),
            }
        }

        pub fn device_close(mut self_: Pin<&mut BoxedAudioIODevice>) {
            self_.close()
        }
    }
}

/// Controls for the system volume.
pub struct SystemAudioVolume;

impl SystemAudioVolume {
    /// Get the current system volume.
    pub fn get_gain() -> f32 {
        juce::get_gain()
    }

    /// Set the system volume.
    pub fn set_gain(gain: f32) {
        juce::set_gain(gain.clamp(0.0, 1.0))
    }

    /// Returns true if the system audio output is muted.
    pub fn is_muted() -> bool {
        juce::is_muted()
    }

    /// Mute the system audio output.
    pub fn mute() {
        juce::set_muted(true);
    }

    /// Unmute the system audio output.
    pub fn unmute() {
        juce::set_muted(false);
    }
}
