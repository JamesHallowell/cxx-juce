use crate::{define_trait, juce_audio_processors::PluginDescription, juce_core::JuceString};
use cxx::UniquePtr;
use std::pin::Pin;

pub use juce::{AudioPluginInstance, AudioProcessor};

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

        type AudioPluginInstance;
        type AudioProcessor;
        type JuceString = crate::juce_core::JuceString;
        type AudioSampleBuffer = crate::juce_audio_basics::AudioSampleBuffer;
        type MidiBuffer = crate::juce_audio_basics::MidiBuffer;
        type PluginDescription = crate::juce_audio_processors::PluginDescription;

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

        #[cxx_name = "processBlockBypassed"]
        fn process_block_bypassed(
            self: Pin<&mut AudioProcessor>,
            audio: Pin<&mut AudioSampleBuffer>,
            midi: Pin<&mut MidiBuffer>,
        );

        #[cxx_name = "getTotalNumInputChannels"]
        fn get_total_num_input_channels(self: &AudioProcessor) -> i32;

        #[cxx_name = "getTotalNumOutputChannels"]
        fn get_total_num_output_channels(self: &AudioProcessor) -> i32;

        #[cxx_name = "getLatencySamples"]
        fn get_latency_samples(self: &AudioProcessor) -> i32;

        #[cxx_name = "setLatencySamples"]
        fn set_latency_samples(self: Pin<&mut AudioProcessor>, new_latency: i32);

        #[cxx_name = "getSampleRate"]
        fn get_sample_rate(self: &AudioProcessor) -> f64;

        #[cxx_name = "getBlockSize"]
        fn get_block_size(self: &AudioProcessor) -> i32;

        #[cxx_name = "isSuspended"]
        fn is_suspended(self: &AudioProcessor) -> bool;

        #[cxx_name = "setNonRealtime"]
        fn set_non_realtime(self: Pin<&mut AudioProcessor>, is_non_realtime: bool);

        #[cxx_name = "fillInPluginDescription"]
        fn fill_in_plugin_description(
            self: &AudioPluginInstance,
            description: Pin<&mut PluginDescription>,
        );
    }

    #[namespace = "cxx_juce"]
    extern "Rust" {
        type AudioPluginImpl;

        #[Self = "AudioPluginImpl"]
        unsafe fn drop(self_: *mut BoxDynAudioPlugin);

        #[Self = "AudioPluginImpl"]
        fn get_name(self_: &BoxDynAudioPlugin) -> JuceString;

        #[Self = "AudioPluginImpl"]
        fn prepare_to_play(self_: &mut BoxDynAudioPlugin, sample_rate: f64, samples_per_block: i32);

        #[Self = "AudioPluginImpl"]
        fn release_resources(self_: &mut BoxDynAudioPlugin);

        #[Self = "AudioPluginImpl"]
        fn process_block(
            self_: &mut BoxDynAudioPlugin,
            audio: Pin<&mut AudioSampleBuffer>,
            midi: Pin<&mut MidiBuffer>,
        );

        #[Self = "AudioPluginImpl"]
        fn get_tail_length_seconds(self_: &BoxDynAudioPlugin) -> f64;

        #[Self = "AudioPluginImpl"]
        fn accepts_midi(self_: &BoxDynAudioPlugin) -> bool;

        #[Self = "AudioPluginImpl"]
        fn produces_midi(self_: &BoxDynAudioPlugin) -> bool;

        #[Self = "AudioPluginImpl"]
        fn has_editor(self_: &BoxDynAudioPlugin) -> bool;

        #[Self = "AudioPluginImpl"]
        fn get_num_programs(self_: &mut BoxDynAudioPlugin) -> i32;

        #[Self = "AudioPluginImpl"]
        fn get_current_program(self_: &mut BoxDynAudioPlugin) -> i32;

        #[Self = "AudioPluginImpl"]
        fn set_current_program(self_: &mut BoxDynAudioPlugin, index: i32);

        #[Self = "AudioPluginImpl"]
        fn get_program_name(self_: &mut BoxDynAudioPlugin, index: i32) -> JuceString;

        #[Self = "AudioPluginImpl"]
        fn change_program_name(self_: &mut BoxDynAudioPlugin, index: i32, new_name: &JuceString);

        #[Self = "AudioPluginImpl"]
        fn fill_in_plugin_description(
            self_: &BoxDynAudioPlugin,
            description: &mut PluginDescription,
        );
    }
}

define_trait! {
    /// A plugin instance.
    AudioPlugin,
    AudioPluginImpl,
    "cxx_juce::BoxDynAudioPlugin",

    /// Get the plugin name.
    fn get_name(&self) -> JuceString;

    /// Prepare the plugin for playback.
    fn prepare_to_play(&mut self, sample_rate: f64, samples_per_block: i32);

    /// Release resources when playback stops.
    fn release_resources(&mut self);

    /// Process an audio block.
    fn process_block(
        &mut self,
        audio: Pin<&mut crate::juce_audio_basics::AudioSampleBuffer>,
        midi: Pin<&mut crate::juce_audio_basics::MidiBuffer>,
    );

    /// Get the tail length in seconds.
    fn get_tail_length_seconds(&self) -> f64;

    /// Returns true if the plugin accepts MIDI input.
    fn accepts_midi(&self) -> bool;

    /// Returns true if the plugin produces MIDI output.
    fn produces_midi(&self) -> bool;

    /// Returns true if the plugin has an editor.
    fn has_editor(&self) -> bool;

    /// Get the number of programs/presets.
    fn get_num_programs(&mut self) -> i32;

    /// Get the current program index.
    fn get_current_program(&mut self) -> i32;

    /// Set the current program index.
    fn set_current_program(&mut self, index: i32);

    /// Get the name of a program.
    fn get_program_name(&mut self, index: i32) -> JuceString;

    /// Change the name of a program.
    fn change_program_name(&mut self, index: i32, new_name: &JuceString);

    /// Fill in a PluginDescription structure with information about this plugin.
    fn fill_in_plugin_description(&self, description: &mut PluginDescription);
}

impl From<Box<dyn AudioPlugin>> for UniquePtr<AudioPluginInstance> {
    fn from(plugin: Box<dyn AudioPlugin>) -> Self {
        juce::wrap_plugin_instance(plugin)
    }
}
