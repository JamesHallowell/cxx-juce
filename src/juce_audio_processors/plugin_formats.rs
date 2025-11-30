use crate::{
    define_juce_type, define_trait,
    juce_audio_processors::{AudioPluginInstance, OwnedArrayPluginDescription, PluginDescription},
    JuceString,
};
use cxx::UniquePtr;
use std::pin::Pin;

define_juce_type! {
    AudioPluginFormatManager,
    layout = juce::AudioPluginFormatManagerLayout,
    cxx_name = "juce::AudioPluginFormatManager",
    default = juce::audio_plugin_format_manager_new,
    drop = juce::audio_plugin_format_manager_drop,
}

impl AudioPluginFormatManager {
    pub fn get_format_ref(&self, index: i32) -> Option<&juce::AudioPluginFormat> {
        let ptr = self.get_format(index);
        unsafe { ptr.as_ref() }
    }

    pub fn get_format_mut(&mut self, index: i32) -> Option<Pin<&mut juce::AudioPluginFormat>> {
        let ptr = self.get_format(index);
        unsafe { ptr.as_mut().map(|ptr| Pin::new_unchecked(ptr)) }
    }

    pub fn add_format(&mut self, format: impl AudioPluginFormat + 'static) {
        let boxed = Box::new(format);
        let wrapped = juce::wrap_audio_plugin_format(boxed);
        unsafe { self.add_format_raw(wrapped.into_raw()) }
    }
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum AudioPluginFormatManagerLayout {
        #[cfg(all(debug_assertions, not(windows)))]
        Size = 32,
        #[cfg(any(not(debug_assertions), windows))]
        Size = 16,
        Alignment = 8,
    }

    unsafe extern "C++" {
        include!("cxx_juce.h");
        include!("cxx_juce_audio_processors/cxx_juce_audio_processors.h");

        type JuceString = crate::JuceString;
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

        #[rust_name = "get_format"]
        #[doc(hidden)]
        fn getFormat(self: &AudioPluginFormatManager, index: i32) -> *mut AudioPluginFormat;

        #[rust_name = "add_format_raw"]
        #[doc(hidden)]
        unsafe fn addFormat(self: &mut AudioPluginFormatManager, format: *mut AudioPluginFormat);

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
    }
}

define_trait! {
    AudioPluginFormat,
    AudioPluginFormatImpl,
    "cxx_juce::BoxDynAudioPluginFormat",

    fn name(&self) -> JuceString;

    fn find_all_types_for_file(
        &mut self,
        results: &mut OwnedArrayPluginDescription,
        file: &JuceString,
    );

    fn create_plugin_instance(
        &mut self,
        description: &PluginDescription,
        sample_rate: f64,
        buffer_size: i32,
    ) -> UniquePtr<AudioPluginInstance>;
}
