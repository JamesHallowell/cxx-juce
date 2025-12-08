use crate::{
    define_juce_type, define_trait,
    juce_audio_processors::{AudioPluginInstance, OwnedArrayPluginDescription, PluginDescription},
    juce_core::{FileSearchPath, JuceString, StringArray},
    JUCE,
};
use cxx::UniquePtr;
use std::pin::Pin;

define_juce_type! {
    AudioPluginFormatManager,
    layout = juce::AudioPluginFormatManagerLayout,
    cxx_name = "juce::AudioPluginFormatManager",
    drop = juce::audio_plugin_format_manager_drop,
}

impl AudioPluginFormatManager {
    pub fn new(_: &JUCE) -> Self {
        juce::audio_plugin_format_manager_new()
    }

    pub fn get_format_ref(&self, index: i32) -> Option<&juce::AudioPluginFormat> {
        let ptr = self.get_format_raw(index);
        unsafe { ptr.as_ref() }
    }

    pub fn get_format_mut(&mut self, index: i32) -> Option<Pin<&mut juce::AudioPluginFormat>> {
        let ptr = self.get_format_raw(index);
        unsafe { ptr.as_mut().map(|ptr| Pin::new_unchecked(ptr)) }
    }

    pub fn add_format(&mut self, format: impl AudioPluginFormat + 'static) {
        let boxed = Box::new(format);
        let wrapped = juce::wrap_audio_plugin_format(boxed);
        unsafe { self.add_format_raw(wrapped.into_raw()) }
    }

    pub fn for_each_format_mut(&mut self, mut func: impl FnMut(Pin<&mut juce::AudioPluginFormat>)) {
        for i in 0..self.get_num_formats() {
            if let Some(format) = self.get_format_mut(i) {
                func(format);
            }
        }
    }
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum AudioPluginFormatManagerLayout {
        Size = 16,
        Alignment = 8,
    }

    unsafe extern "C++" {
        include!("cxx_juce.h");

        type JuceString = crate::juce_core::JuceString;
        type PluginDescription = crate::juce_audio_processors::PluginDescription;
        type OwnedArrayPluginDescription =
            crate::juce_audio_processors::OwnedArrayPluginDescription;
        type AudioPluginFormatManager = super::AudioPluginFormatManager;
        type AudioPluginFormat;
        type AudioPluginInstance = crate::juce_audio_processors::AudioPluginInstance;
        type FileSearchPath = crate::juce_core::FileSearchPath;
        type StringArray = crate::juce_core::StringArray;

        #[namespace = "cxx_juce"]
        type BoxDynAudioPluginFormat = Box<dyn super::AudioPluginFormat>;

        #[namespace = "cxx_juce"]
        #[cxx_name = "wrap"]
        fn wrap_audio_plugin_format(
            format: BoxDynAudioPluginFormat,
        ) -> UniquePtr<AudioPluginFormat>;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn audio_plugin_format_manager_new() -> AudioPluginFormatManager;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn audio_plugin_format_manager_drop(self_: &mut AudioPluginFormatManager);

        #[rust_name = "add_default_formats"]
        fn addDefaultFormats(self: &mut AudioPluginFormatManager);

        #[rust_name = "get_num_formats"]
        fn getNumFormats(self: &AudioPluginFormatManager) -> i32;

        #[cxx_name = "getFormat"]
        #[doc(hidden)]
        fn get_format_raw(self: &AudioPluginFormatManager, index: i32) -> *mut AudioPluginFormat;

        #[doc(hidden)]
        #[cxx_name = "addFormat"]
        unsafe fn add_format_raw(
            self: &mut AudioPluginFormatManager,
            format: *mut AudioPluginFormat,
        );

        #[cxx_name = "getName"]
        fn get_name(self: &AudioPluginFormat) -> JuceString;

        #[cxx_name = "findAllTypesForFile"]
        fn find_all_types_for_file(
            self: Pin<&mut AudioPluginFormat>,
            result: &mut OwnedArrayPluginDescription,
            file: &JuceString,
        );

        #[cxx_name = "createInstanceFromDescription"]
        fn create_instance_from_description(
            self: Pin<&mut AudioPluginFormat>,
            description: &PluginDescription,
            sample_rate: f64,
            buffer_size: i32,
            error: &mut JuceString,
        ) -> UniquePtr<AudioPluginInstance>;

        #[cxx_name = "fileMightContainThisPluginType"]
        fn file_might_contain_this_plugin_type(
            self: Pin<&mut AudioPluginFormat>,
            file_or_identifier: &JuceString,
        ) -> bool;

        #[cxx_name = "getNameOfPluginFromIdentifier"]
        fn get_name_of_plugin_from_identifier(
            self: Pin<&mut AudioPluginFormat>,
            file_or_identifier: &JuceString,
        ) -> JuceString;

        #[cxx_name = "pluginNeedsRescanning"]
        fn plugin_need_rescanning(
            self: Pin<&mut AudioPluginFormat>,
            plugin: &PluginDescription,
        ) -> bool;

        #[cxx_name = "doesPluginStillExist"]
        fn does_plugin_still_exist(
            self: Pin<&mut AudioPluginFormat>,
            plugin: &PluginDescription,
        ) -> bool;

        #[cxx_name = "canScanForPlugins"]
        fn can_scan_for_plugins(self: &AudioPluginFormat) -> bool;

        #[cxx_name = "isTrivialToScan"]
        fn is_trivial_to_scan(self: &AudioPluginFormat) -> bool;

        #[cxx_name = "searchPathsForPlugins"]
        fn search_paths_for_plugins(
            self: Pin<&mut AudioPluginFormat>,
            directories_to_search: &FileSearchPath,
            recursive: bool,
            allow_async_plugins: bool,
        ) -> StringArray;

        #[cxx_name = "getDefaultLocationsToSearch"]
        fn get_default_locations_to_search(self: Pin<&mut AudioPluginFormat>) -> FileSearchPath;
    }

    #[namespace = "cxx_juce"]
    extern "Rust" {
        type AudioPluginFormatImpl;

        #[Self = "AudioPluginFormatImpl"]
        unsafe fn drop(format: *mut BoxDynAudioPluginFormat);

        #[Self = "AudioPluginFormatImpl"]
        fn name(format: &BoxDynAudioPluginFormat) -> JuceString;

        #[Self = "AudioPluginFormatImpl"]
        fn find_all_types_for_file(
            format: &mut BoxDynAudioPluginFormat,
            results: &mut OwnedArrayPluginDescription,
            file: &JuceString,
        );

        #[Self = "AudioPluginFormatImpl"]
        fn create_plugin_instance(
            format: &mut BoxDynAudioPluginFormat,
            description: &PluginDescription,
            sample_rate: f64,
            buffer_size: i32,
        ) -> UniquePtr<AudioPluginInstance>;

        #[Self = "AudioPluginFormatImpl"]
        fn file_might_contain_this_plugin_type(
            format: &BoxDynAudioPluginFormat,
            file_or_identifier: &JuceString,
        ) -> bool;

        #[Self = "AudioPluginFormatImpl"]
        fn get_name_of_plugin_from_identifier(
            format: &BoxDynAudioPluginFormat,
            file_or_identifier: &JuceString,
        ) -> JuceString;

        #[Self = "AudioPluginFormatImpl"]
        fn plugin_needs_rescanning(
            format: &BoxDynAudioPluginFormat,
            plugin: &PluginDescription,
        ) -> bool;

        #[Self = "AudioPluginFormatImpl"]
        fn does_plugin_still_exist(
            format: &BoxDynAudioPluginFormat,
            plugin: &PluginDescription,
        ) -> bool;

        #[Self = "AudioPluginFormatImpl"]
        fn can_scan_for_plugins(format: &BoxDynAudioPluginFormat) -> bool;

        #[Self = "AudioPluginFormatImpl"]
        fn is_trivial_to_scan(format: &BoxDynAudioPluginFormat) -> bool;

        #[Self = "AudioPluginFormatImpl"]
        fn search_paths_for_plugins(
            format: &mut BoxDynAudioPluginFormat,
            directories_to_search: &FileSearchPath,
            recursive: bool,
            allow_async_plugins: bool,
        ) -> StringArray;

        #[Self = "AudioPluginFormatImpl"]
        fn get_default_locations_to_search(format: &mut BoxDynAudioPluginFormat) -> FileSearchPath;

        #[Self = "AudioPluginFormatImpl"]
        fn requires_unblocked_message_thread_during_creation(
            format: &BoxDynAudioPluginFormat,
            description: &PluginDescription,
        ) -> bool;
    }
}

define_trait! {
    AudioPluginFormat,
    AudioPluginFormatImpl,
    "cxx_juce::BoxDynAudioPluginFormat",

    /// Returns the name of this plugin format.
    fn name(&self) -> JuceString;

    /// Searches a file or identifier to find all the plugin types contained within it.
    fn find_all_types_for_file(
        &mut self,
        results: &mut OwnedArrayPluginDescription,
        file: &JuceString,
    );

    /// Creates an instance of a plugin from a description.
    fn create_plugin_instance(
        &mut self,
        description: &PluginDescription,
        sample_rate: f64,
        buffer_size: i32,
    ) -> UniquePtr<AudioPluginInstance>;

    /// Returns true if the given file or identifier might contain plugins of this format.
    fn file_might_contain_this_plugin_type(&self, file_or_identifier: &JuceString) -> bool;

    /// Tries to extract the name of a plugin from a file or identifier.
    fn get_name_of_plugin_from_identifier(&self, file_or_identifier: &JuceString) -> JuceString;

    /// Returns true if the plugin needs to be rescanned.
    fn plugin_needs_rescanning(&self, plugin: &PluginDescription) -> bool;

    /// Returns true if the plugin still exists on disk or in the system.
    fn does_plugin_still_exist(&self, plugin: &PluginDescription) -> bool;

    /// Returns true if this format can scan for plugins.
    fn can_scan_for_plugins(&self) -> bool;

    /// Returns true if this format is trivial to scan (e.g., no filesystem access required).
    fn is_trivial_to_scan(&self) -> bool;

    /// Searches the given directories for plugins of this format.
    fn search_paths_for_plugins(
        &mut self,
        directories_to_search: &FileSearchPath,
        recursive: bool,
        allow_async_plugins: bool,
    ) -> StringArray;

    /// Returns the default locations to search for plugins of this format.
    fn get_default_locations_to_search(&mut self) -> FileSearchPath;

    /// Returns true if the plugin requires an unblocked message thread during creation.
    fn requires_unblocked_message_thread_during_creation(&self, description: &PluginDescription) -> bool;
}
