use crate::define_juce_type;

define_juce_type! {
    MidiBuffer,
    layout = juce::MidiBufferLayout,
    cxx_name = "juce::MidiBuffer",
    default = juce::midi_buffer_new,
    drop = juce::midi_buffer_drop,
}

define_juce_type! {
    MidiMessage,
    layout = juce::MidiMessageLayout,
    cxx_name = "juce::MidiMessage",
    drop = juce::midi_message_drop,
    clone = juce::midi_message_clone,
}

unsafe impl Send for MidiMessage {}

impl std::fmt::Debug for MidiMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_description())
    }
}

define_juce_type! {
    MidiFile,
    layout = juce::MidiFileLayout,
    cxx_name = "juce::MidiFile",
    default = juce::midi_file_new,
    drop = juce::midi_file_drop,
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum MidiBufferLayout {
        Size = 16,
        Alignment = 8,
    }

    enum MidiMessageLayout {
        Size = 24,
        Alignment = 8,
    }

    enum MidiFileLayout {
        Size = 24,
        Alignment = 8,
    }

    unsafe extern "C++" {
        include!("cxx_juce.h");

        type JuceString = crate::juce_core::JuceString;

        type MidiBuffer = super::MidiBuffer;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn midi_buffer_new() -> MidiBuffer;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn midi_buffer_drop(buffer: &mut MidiBuffer);

        fn clear(self: &mut MidiBuffer);

        #[cxx_name = "isEmpty"]
        fn is_empty(self: &MidiBuffer) -> bool;

        #[cxx_name = "getNumEvents"]
        fn get_num_events(self: &MidiBuffer) -> i32;

        #[cxx_name = "addEvent"]
        fn add_event(self: &mut MidiBuffer, message: &MidiMessage, sample_number: i32) -> bool;

        #[cxx_name = "clear"]
        fn clear_range(self: &mut MidiBuffer, start_sample: i32, num_samples: i32);

        type MidiMessage = super::MidiMessage;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn midi_message_drop(msg: &mut MidiMessage);

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn midi_message_clone(message: &MidiMessage) -> MidiMessage;

        #[cxx_name = "noteOn"]
        #[Self = "MidiMessage"]
        fn note_on(channel: i32, note_number: i32, velocity: f32) -> MidiMessage;

        #[cxx_name = "noteOff"]
        #[Self = "MidiMessage"]
        fn note_off(channel: i32, note_number: i32, velocity: f32) -> MidiMessage;

        #[cxx_name = "getDescription"]
        fn get_description(self: &MidiMessage) -> JuceString;

        #[cxx_name = "isNoteOn"]
        fn is_note_on(self: &MidiMessage, return_true_for_note_on_velocity_zero: bool) -> bool;

        #[cxx_name = "isNoteOff"]
        fn is_note_off(self: &MidiMessage, return_true_for_note_on_velocity_zero: bool) -> bool;

        #[cxx_name = "getChannel"]
        fn get_channel(self: &MidiMessage) -> i32;

        #[cxx_name = "getNoteNumber"]
        fn get_note_number(self: &MidiMessage) -> i32;

        #[cxx_name = "getVelocity"]
        fn get_velocity(self: &MidiMessage) -> u8;

        #[cxx_name = "getFloatVelocity"]
        fn get_float_velocity(self: &MidiMessage) -> f32;

        #[cxx_name = "getTimeStamp"]
        fn get_time_stamp(self: &MidiMessage) -> f64;

        type MidiFile = super::MidiFile;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn midi_file_drop(file: &mut MidiFile);

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn midi_file_new() -> MidiFile;

        #[cxx_name = "getNumTracks"]
        fn get_num_tracks(self: &MidiFile) -> i32;

        #[cxx_name = "getTimeFormat"]
        fn get_time_format(self: &MidiFile) -> i16;

        #[cxx_name = "setTicksPerQuarterNote"]
        fn set_ticks_per_quarter_note(self: &mut MidiFile, ticks_per_quarter_note: i32);

        #[cxx_name = "setSmpteTimeFormat"]
        fn set_smpte_time_format(
            self: &mut MidiFile,
            frames_per_second: i32,
            sub_frame_divisor: i32,
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creating_note_on_message() {
        let msg = MidiMessage::note_on(1, 60, 0.8);
        assert!(msg.is_note_on(false));
        assert_eq!(msg.get_channel(), 1);
        assert_eq!(msg.get_note_number(), 60);
    }

    #[test]
    fn creating_note_off_message() {
        let msg = MidiMessage::note_off(1, 60, 0.5);
        assert!(msg.is_note_off(false));
        assert_eq!(msg.get_channel(), 1);
        assert_eq!(msg.get_note_number(), 60);
    }

    #[test]
    fn creating_midi_file() {
        let mut file = MidiFile::default();
        assert_eq!(file.get_num_tracks(), 0);

        file.set_ticks_per_quarter_note(480);
        assert_eq!(file.get_time_format(), 480);
    }

    #[test]
    fn working_with_midi_buffer() {
        let mut buffer = MidiBuffer::default();
        assert!(buffer.is_empty());
        assert_eq!(buffer.get_num_events(), 0);

        let msg1 = MidiMessage::note_on(1, 60, 0.8);
        let msg2 = MidiMessage::note_off(1, 60, 0.5);

        buffer.add_event(&msg1, 0);
        buffer.add_event(&msg2, 100);

        assert!(!buffer.is_empty());
        assert_eq!(buffer.get_num_events(), 2);

        buffer.clear();
        assert!(buffer.is_empty());
        assert_eq!(buffer.get_num_events(), 0);
    }

    #[test]
    fn debugging_midi_message() {
        assert_eq!(
            format!("{:?}", MidiMessage::note_on(1, 60, 0.8)),
            "Note on C3 Velocity 102 Channel 1"
        );
    }
}
