use cxx::UniquePtr;
use cxx_juce::{
    juce_audio_processors::{
        AudioPlugin, AudioPluginFormat, AudioPluginFormatManager, AudioPluginInstance,
        OwnedArrayPluginDescription, PluginDescription,
    },
    juce_core::JuceString,
    JUCE,
};

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
}

struct MockPlugin;

impl AudioPlugin for MockPlugin {
    fn get_name(&self) -> JuceString {
        JuceString::new("Mock Plugin")
    }
}

#[test]
fn creating_an_instance_of_an_audio_plugin() {
    let _juce = JUCE::initialise();
    let mut mgr = AudioPluginFormatManager::default();

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
