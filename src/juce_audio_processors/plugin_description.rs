use crate::{
    define_array_into_iter, define_juce_type,
    juce_core::{ArrayLayout, JuceString, Time},
};
use cxx::UniquePtr;

define_juce_type! {
    #[derive(Debug)]
    /// Description of a plugin.
    PluginDescription,
    fields = {
        /// The plugin name.
        pub name: JuceString = {
            offset = juce::PluginDescriptionLayout::NameOffset,
            with = with_name,
        },
        /// A more descriptive name for the plugin.
        pub descriptive_name: JuceString = {
            offset = juce::PluginDescriptionLayout::DescriptiveNameOffset,
            with = with_descriptive_name,
        },
        /// The name of the plugin format (e.g. "VST3", "AudioUnit").
        pub plugin_format_name: JuceString = {
            offset = juce::PluginDescriptionLayout::PluginFormatNameOffset,
            with = with_plugin_format_name,
        },
        /// The plugin category.
        pub category: JuceString = {
            offset = juce::PluginDescriptionLayout::CategoryOffset,
            with = with_category,
        },
        /// The plugin manufacturer name.
        pub manufacturer_name: JuceString = {
            offset = juce::PluginDescriptionLayout::ManufacturerNameOffset,
            with = with_manufacturer_name,
        },
        /// The plugin version string.
        pub version: JuceString = {
            offset = juce::PluginDescriptionLayout::VersionOffset,
            with = with_version,
        },
        /// The file path or unique identifier of the plugin.
        pub file_or_identifier: JuceString = {
            offset = juce::PluginDescriptionLayout::FileOrIdentifierOffset,
            with = with_file_or_identifier,
        },
        /// The last modification time of the plugin file.
        pub last_file_mod_time: Time = {
            offset = juce::PluginDescriptionLayout::LastFileModTimeOffset,
            with = with_last_file_mod_time,
            get = last_file_mod_time,
        },
        /// The last time the plugin info was updated.
        pub last_info_update_time: Time = {
            offset = juce::PluginDescriptionLayout::LastInfoUpdateTimeOffset,
            with = with_last_info_update_time,
        },
        deprecated_uid: i32 = {
            offset = juce::PluginDescriptionLayout::DeprecatedUidOffset,
        },
        /// The unique ID of the plugin.
        pub unique_id: i32 = {
            offset = juce::PluginDescriptionLayout::UniqueIdOffset,
            with = with_unique_id,
        },
        /// Whether the plugin is an instrument.
        pub is_instrument: bool = {
            offset = juce::PluginDescriptionLayout::IsInstrumentOffset,
            with = with_is_instrument,
        },
        /// The number of input channels.
        pub num_input_channels: i32 = {
            offset = juce::PluginDescriptionLayout::NumInputChannelsOffset,
            with = with_num_input_channels,
        },
        /// The number of output channels.
        pub num_output_channels: i32 = {
            offset = juce::PluginDescriptionLayout::NumOutputChannelsOffset,
            with = with_num_output_channels,
        },
        /// Whether the plugin has a shared container.
        pub has_shared_container: bool = {
            offset = juce::PluginDescriptionLayout::HasSharedContainerOffset,
            with = with_has_shared_container,
        },
        /// Whether the plugin has an ARA extension.
        pub has_ara_extension: bool = {
            offset = juce::PluginDescriptionLayout::HasAraExtensionOffset,
            with = with_has_ara_extension,
        },
    },
    layout = juce::PluginDescriptionLayout,
    cxx_name = "juce::PluginDescription",
    default = juce::plugin_description_new,
    clone = juce::plugin_description_clone,
}

define_juce_type! {
    /// An owned array of [`PluginDescription`].
    OwnedArrayPluginDescription,
    layout = ArrayLayout,
    cxx_name = "juce::OwnedArrayPluginDescription",
    default = juce::owned_array_plugin_description_new,
    drop = juce::owned_array_plugin_description_drop,
}

define_array_into_iter! {
    OwnedArrayPluginDescription => OwnedArrayPluginDescriptionIter,
    ref PluginDescription,
    OwnedArrayPluginDescription::get
}

impl OwnedArrayPluginDescription {
    /// Adds a plugin description to the array, taking ownership.
    pub fn add(&mut self, description: PluginDescription) {
        let ptr = UniquePtr::new(description).into_raw();
        unsafe { self.add_raw(ptr) };
    }

    /// Returns a reference to the description at the given index, or [`None`] if out of bounds.
    pub fn get(&self, index: i32) -> Option<&PluginDescription> {
        let result = juce::owned_array_plugin_description_get(self, index);
        unsafe { result.as_ref() }
    }
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum PluginDescriptionLayout {
        Size = 96,
        Alignment = 8,

        NameOffset = 0,
        DescriptiveNameOffset = 8,
        PluginFormatNameOffset = 16,
        CategoryOffset = 24,
        ManufacturerNameOffset = 32,
        VersionOffset = 40,
        FileOrIdentifierOffset = 48,
        LastFileModTimeOffset = 56,
        LastInfoUpdateTimeOffset = 64,
        DeprecatedUidOffset = 72,
        UniqueIdOffset = 76,
        IsInstrumentOffset = 80,
        NumInputChannelsOffset = 84,
        NumOutputChannelsOffset = 88,
        HasSharedContainerOffset = 92,
        HasAraExtensionOffset = 93,
    }

    impl UniquePtr<PluginDescription> {}

    unsafe extern "C++" {
        include!("cxx_juce.h");
        include!("cxx_juce_audio_processors/cxx_juce_audio_processors.h");

        type PluginDescription = super::PluginDescription;
        type OwnedArrayPluginDescription = super::OwnedArrayPluginDescription;
        type JuceString = crate::juce_core::JuceString;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn plugin_description_new() -> PluginDescription;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn plugin_description_clone(plugin: &PluginDescription) -> PluginDescription;

        /// Creates a unique identifier string for this plugin.
        #[cxx_name = "createIdentifierString"]
        fn create_identifier_string(self: &PluginDescription) -> JuceString;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn owned_array_plugin_description_new() -> OwnedArrayPluginDescription;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn owned_array_plugin_description_drop(self_: &mut OwnedArrayPluginDescription);

        /// Adds a raw plugin description pointer to the array.
        #[cxx_name = "add"]
        unsafe fn add_raw(
            self: &mut OwnedArrayPluginDescription,
            description: *mut PluginDescription,
        ) -> *mut PluginDescription;

        #[namespace = "cxx_juce"]
        #[cxx_name = "index"]
        fn owned_array_plugin_description_get(
            self_: &OwnedArrayPluginDescription,
            index: i32,
        ) -> *mut PluginDescription;

        /// Returns the number of descriptions in the array.
        #[cxx_name = "size"]
        fn len(self: &OwnedArrayPluginDescription) -> i32;

        /// Returns a pointer to the raw data.
        fn data(self: &OwnedArrayPluginDescription) -> *const *mut PluginDescription;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn creating_plugin_descriptions() {
        let a = PluginDescription::default().with_name("A");
        let b = PluginDescription::default().with_name("B");
        let c = PluginDescription::default().with_name("C");

        let mut array = OwnedArrayPluginDescription::default();
        array.add(a);
        array.add(b);
        array.add(c);

        assert_eq!(array.get(0).unwrap().name, "A");
        assert_eq!(array.get(1).unwrap().name, "B");
        assert_eq!(array.get(2).unwrap().name, "C");
    }

    #[test]
    fn getting_modification_timestamp() {
        assert_eq!(
            PluginDescription::default()
                .with_last_file_mod_time(UNIX_EPOCH)
                .last_file_mod_time,
            Time::from(UNIX_EPOCH)
        );

        assert_eq!(
            PluginDescription::default()
                .with_last_file_mod_time(UNIX_EPOCH + Duration::from_millis(100))
                .last_file_mod_time,
            Time::from(UNIX_EPOCH + Duration::from_millis(100))
        );

        assert_eq!(
            PluginDescription::default()
                .with_last_file_mod_time(UNIX_EPOCH - Duration::from_millis(100))
                .last_file_mod_time,
            Time::from(UNIX_EPOCH - Duration::from_millis(100))
        );
    }
}
