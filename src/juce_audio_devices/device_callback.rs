use crate::{
    define_trait, juce_audio_basics::AudioSampleBuffer, juce_audio_devices::AudioIODevice,
};
use cxx::UniquePtr;
use std::pin::Pin;

pub use juce::{AudioIODeviceCallback, BoxDynAudioDeviceCallback};

#[cxx::bridge(namespace = "cxx_juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        #[namespace = "juce"]
        type AudioIODeviceCallback;

        type BoxDynAudioDeviceCallback = Box<dyn super::AudioDeviceCallback>;

        #[cxx_name = "wrap"]
        fn wrap_audio_device_callback(
            callback: BoxDynAudioDeviceCallback,
        ) -> UniquePtr<AudioIODeviceCallback>;

        #[namespace = "juce"]
        type AudioIODevice = crate::juce_audio_devices::AudioIODevice;

        #[namespace = "juce"]
        type AudioSampleBuffer = crate::juce_audio_basics::AudioSampleBuffer;
    }

    #[namespace = "cxx_juce"]
    extern "Rust" {
        type AudioDeviceCallbackImpl;

        #[Self = "AudioDeviceCallbackImpl"]
        unsafe fn drop(callback: *mut BoxDynAudioDeviceCallback);

        #[Self = "AudioDeviceCallbackImpl"]
        fn about_to_start(
            callback: &mut BoxDynAudioDeviceCallback,
            device: Pin<&mut AudioIODevice>,
        );

        #[Self = "AudioDeviceCallbackImpl"]
        fn process_block(
            callback: &mut BoxDynAudioDeviceCallback,
            input: &AudioSampleBuffer,
            output: Pin<&mut AudioSampleBuffer>,
        );

        #[Self = "AudioDeviceCallbackImpl"]
        fn stopped(callback: &mut BoxDynAudioDeviceCallback);
    }
}

define_trait! {
    /// A trait that can be implemented to receive audio callbacks.
    ///
    /// Types that implement this trait can be registered with [`AudioDeviceManager::add_audio_callback`].
    ///
    /// This trait requires that implementors are [`Send`] because the callbacks will occur on the audio thread.
    AudioDeviceCallback: Send,
    AudioDeviceCallbackImpl,
    "cxx_juce::BoxDynAudioDeviceCallback",

    /// Called when the audio device is about to start.
    fn about_to_start(&mut self, device: Pin<&mut AudioIODevice>);

    /// Process a block of incoming and outgoing audio.
    fn process_block(&mut self, input: &AudioSampleBuffer, output: Pin<&mut AudioSampleBuffer>);

    /// Called when the audio device has stopped.
    fn stopped(&mut self);
}

impl From<Box<dyn AudioDeviceCallback>> for UniquePtr<AudioIODeviceCallback> {
    fn from(audio_device: Box<dyn AudioDeviceCallback>) -> Self {
        juce::wrap_audio_device_callback(audio_device)
    }
}
