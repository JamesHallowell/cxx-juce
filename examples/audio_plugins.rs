use cxx_juce::{
    juce_audio_processors::{AudioPluginFormatManager, OwnedArrayPluginDescription},
    JUCE,
};

fn main() {
    let juce = JUCE::initialise();

    let mut manager = AudioPluginFormatManager::new(&juce);
    manager.add_default_formats();

    manager.for_each_format_mut(|mut format| {
        println!("🔎 Searching for {} plugins...", format.get_name());

        let default_locations = format.as_mut().get_default_locations_to_search();
        let identifiers = format
            .as_mut()
            .search_paths_for_plugins(&default_locations, true, true);

        let mut descriptions = OwnedArrayPluginDescription::default();
        for identifier in identifiers {
            format
                .as_mut()
                .find_all_types_for_file(&mut descriptions, &identifier);
        }

        for description in &descriptions {
            println!(
                "  - {} (v{}), {}",
                description.name, description.version, description.manufacturer_name
            );
        }
    });
}
