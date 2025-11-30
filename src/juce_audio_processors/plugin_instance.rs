use crate::{define_trait, JuceString};
use cxx::UniquePtr;
use std::pin::Pin;

use crate::juce_audio_processors::plugin_instance::juce::AudioProcessor;
pub use juce::AudioPluginInstance;

impl AudioPluginInstance {
    pub fn cast(&self) -> &AudioProcessor {
        juce::audio_plugin_to_processor(self)
    }

    pub fn cast_mut(self: Pin<&mut Self>) -> Pin<&mut AudioProcessor> {
        juce::audio_plugin_to_processor_mut(self)
    }
}

impl AudioProcessor {
    pub fn get_name(&self) -> JuceString {
        juce::audio_processor_get_name(self)
    }
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");
        include!("cxx_juce_audio_processors/cxx_juce_audio_processors.h");

        type AudioPluginInstance;
        type AudioProcessor;
        type JuceString = crate::JuceString;
        type AudioSampleBuffer = crate::juce_audio_basics::AudioSampleBuffer;
        type MidiBuffer = crate::juce_audio_basics::MidiBuffer;

        #[namespace = "cxx_juce"]
        type BoxDynAudioPlugin = Box<dyn super::AudioPlugin>;

        #[namespace = "cxx_juce"]
        #[cxx_name = "wrap"]
        fn wrap_plugin_instance(plugin: BoxDynAudioPlugin) -> UniquePtr<AudioPluginInstance>;

        #[namespace = "cxx_juce"]
        #[cxx_name = "derivedCast"]
        fn audio_plugin_to_processor(plugin: &AudioPluginInstance) -> &AudioProcessor;

        #[namespace = "cxx_juce"]
        #[cxx_name = "derivedCastMut"]
        fn audio_plugin_to_processor_mut(
            plugin: Pin<&mut AudioPluginInstance>,
        ) -> Pin<&mut AudioProcessor>;

        #[namespace = "cxx_juce"]
        #[cxx_name = "audioProcessorGetName"]
        fn audio_processor_get_name(self_: &AudioProcessor) -> JuceString;

        #[cxx_name = "prepareToPlay"]
        fn prepare_to_play(
            self: Pin<&mut AudioProcessor>,
            sample_rate: f64,
            samples_per_block: i32,
        );

        #[cxx_name = "releaseResources"]
        fn release_resources(self: Pin<&mut AudioProcessor>);

        #[cxx_name = "processBlock"]
        fn process_block(
            self: Pin<&mut AudioProcessor>,
            audio: Pin<&mut AudioSampleBuffer>,
            midi: Pin<&mut MidiBuffer>,
        );
    }

    #[namespace = "cxx_juce"]
    extern "Rust" {
        type AudioPluginImpl;

        #[Self = "AudioPluginImpl"]
        unsafe fn drop(self_: *mut BoxDynAudioPlugin);

        #[Self = "AudioPluginImpl"]
        fn get_name(self_: &BoxDynAudioPlugin) -> JuceString;
    }
}

define_trait! {
    /// A plugin instance.
    AudioPlugin,
    AudioPluginImpl,
    "cxx_juce::BoxDynAudioPlugin",

    /// Get the plugin name.
    fn get_name(&self) -> JuceString;
}

impl From<Box<dyn AudioPlugin>> for UniquePtr<AudioPluginInstance> {
    fn from(plugin: Box<dyn AudioPlugin>) -> Self {
        juce::wrap_plugin_instance(plugin)
    }
}
