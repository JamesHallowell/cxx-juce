use cxx::UniquePtr;
use cxx_juce::juce_audio_basics::{AudioSampleBuffer, MidiBuffer};
use cxx_juce::juce_core::{FileSearchPath, StringArray};
use cxx_juce::{
    juce_audio_processors::{
        AudioPlugin, AudioPluginFormat, AudioPluginFormatManager, AudioPluginInstance,
        OwnedArrayPluginDescription, PluginDescription,
    },
    juce_core::JuceString,
    JUCE,
};
use std::pin::Pin;

struct MockPluginFormat;

impl AudioPluginFormat for MockPluginFormat {
    fn name(&self) -> JuceString {
        JuceString::new("Mock Plugin Format")
    }

    fn find_all_types_for_file(
        &mut self,
        results: &mut OwnedArrayPluginDescription,
        _: &JuceString,
    ) {
        let description = PluginDescription::default()
            .with_name("Mock Plugin")
            .with_manufacturer_name("Test Inc.")
            .with_category("Mock");

        results.add(description);
    }

    fn create_plugin_instance(
        &mut self,
        description: &PluginDescription,
        _sample_rate: f64,
        _buffer_size: i32,
    ) -> UniquePtr<AudioPluginInstance> {
        if description.name == "Mock Plugin" {
            let plugin: Box<dyn AudioPlugin> = Box::new(MockPlugin);
            return plugin.into();
        }

        UniquePtr::null()
    }

    fn file_might_contain_this_plugin_type(&self, _: &JuceString) -> bool {
        unimplemented!()
    }

    fn get_name_of_plugin_from_identifier(&self, _: &JuceString) -> JuceString {
        unimplemented!()
    }

    fn plugin_needs_rescanning(&self, _: &PluginDescription) -> bool {
        unimplemented!()
    }

    fn does_plugin_still_exist(&self, _: &PluginDescription) -> bool {
        unimplemented!()
    }

    fn can_scan_for_plugins(&self) -> bool {
        unimplemented!()
    }

    fn is_trivial_to_scan(&self) -> bool {
        unimplemented!()
    }

    fn search_paths_for_plugins(&mut self, _: &FileSearchPath, _: bool, _: bool) -> StringArray {
        unimplemented!()
    }

    fn get_default_locations_to_search(&mut self) -> FileSearchPath {
        unimplemented!()
    }

    fn requires_unblocked_message_thread_during_creation(&self, _: &PluginDescription) -> bool {
        false
    }
}

struct MockPlugin;

impl AudioPlugin for MockPlugin {
    fn get_name(&self) -> JuceString {
        JuceString::new("Mock Plugin")
    }

    fn prepare_to_play(&mut self, _sample_rate: f64, _samples_per_block: i32) {}

    fn release_resources(&mut self) {}

    fn process_block(&mut self, _audio: Pin<&mut AudioSampleBuffer>, _midi: Pin<&mut MidiBuffer>) {}

    fn get_tail_length_seconds(&self) -> f64 {
        0.0
    }

    fn accepts_midi(&self) -> bool {
        false
    }

    fn produces_midi(&self) -> bool {
        false
    }

    fn has_editor(&self) -> bool {
        false
    }

    fn get_num_programs(&mut self) -> i32 {
        1
    }

    fn get_current_program(&mut self) -> i32 {
        0
    }

    fn set_current_program(&mut self, _index: i32) {}

    fn get_program_name(&mut self, _index: i32) -> JuceString {
        JuceString::new("Default")
    }

    fn change_program_name(&mut self, _index: i32, _new_name: &JuceString) {}

    fn fill_in_plugin_description(&self, _: &mut PluginDescription) {}
}

#[test]
fn creating_an_instance_of_an_audio_plugin() {
    let juce = JUCE::initialise();
    let mut mgr = AudioPluginFormatManager::new(&juce);

    mgr.add_format(MockPluginFormat);
    assert_eq!(mgr.get_num_formats(), 1);

    let mut format = mgr.get_format_mut(0).expect("no format found");

    let mut results = OwnedArrayPluginDescription::default();
    format
        .as_mut()
        .find_all_types_for_file(&mut results, &JuceString::default());
    assert_eq!(results.size(), 1);

    let description = results.get(0).expect("no description found");
    assert_eq!(description.name, "Mock Plugin");
    assert_eq!(description.category, "Mock");

    let mut instance = format.create_instance_from_description(
        description,
        48_000.0,
        512,
        &mut JuceString::default(),
    );

    let processor = instance.pin_mut().cast_mut();
    assert_eq!(processor.get_name(), "Mock Plugin");
}
