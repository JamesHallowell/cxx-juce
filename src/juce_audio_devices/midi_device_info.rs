use crate::{
    define_array_type, define_juce_type,
    juce_core::{ArrayLayout, JuceString},
};

define_juce_type! {
    #[derive(Debug)]
    /// Information about a MIDI device
    MidiDeviceInfo,
    fields = {
        pub name: JuceString = {
            offset = juce::MidiDeviceInfoLayout::NameOffset,
        },
        pub identifier: JuceString = {
            offset = juce::MidiDeviceInfoLayout::IdentifierOffset,
        },
    },
    layout = juce::MidiDeviceInfoLayout,
    cxx_name = "juce::MidiDeviceInfo",
    default = juce::midi_device_info_new,
    clone = juce::midi_device_info_clone,
}

impl std::fmt::Display for MidiDeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.identifier)
    }
}

define_juce_type! {
    MidiDeviceInfoArray,
    layout = ArrayLayout,
    cxx_name = "juce::MidiDeviceInfoArray",
    drop = juce::midi_device_info_array_drop,
}

define_array_type! {
    MidiDeviceInfoArray,
    MidiDeviceInfo,
    MidiDeviceInfoArrayIter,
    MidiDeviceInfoArrayIterRef,
    data = MidiDeviceInfoArray::data,
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum MidiDeviceInfoLayout {
        Size = 16,
        Alignment = 8,

        NameOffset = 0,
        IdentifierOffset = 8,
    }

    unsafe extern "C++" {
        include!("cxx_juce.h");

        type JuceString = crate::juce_core::JuceString;
        type MidiDeviceInfo = super::MidiDeviceInfo;
        type MidiDeviceInfoArray = super::MidiDeviceInfoArray;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn midi_device_info_new() -> MidiDeviceInfo;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn midi_device_info_clone(info: &MidiDeviceInfo) -> MidiDeviceInfo;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn midi_device_info_array_drop(self_: &mut MidiDeviceInfoArray);

        #[cxx_name = "getRawDataPointer"]
        fn data(self: &MidiDeviceInfoArray) -> *const MidiDeviceInfo;

        #[cxx_name = "size"]
        fn len(self: &MidiDeviceInfoArray) -> i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_midi_device_info() {
        let info = MidiDeviceInfo::default();
        assert_eq!(info.name, "");
        assert_eq!(info.identifier, "");
    }
}
