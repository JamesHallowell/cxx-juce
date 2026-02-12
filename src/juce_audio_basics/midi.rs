use crate::define_juce_type;

define_juce_type! {
    /// A buffer for holding a sequence of timestamped MIDI events.
    MidiBuffer,
    layout = juce::MidiBufferLayout,
    cxx_name = "juce::MidiBuffer",
    default = juce::midi_buffer_new,
    drop = juce::midi_buffer_drop,
}

define_juce_type! {
    /// A single MIDI message.
    MidiMessage,
    layout = juce::MidiMessageLayout,
    cxx_name = "juce::MidiMessage",
    drop = juce::midi_message_drop,
    clone = juce::midi_message_clone,
    send,
    debug = MidiMessage::get_description,
}

define_juce_type! {
    /// A MIDI file.
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

        /// Removes all events from the buffer.
        fn clear(self: &mut MidiBuffer);

        /// Returns `true` if the buffer contains no events.
        #[cxx_name = "isEmpty"]
        fn is_empty(self: &MidiBuffer) -> bool;

        /// Returns the number of events in the buffer.
        #[cxx_name = "getNumEvents"]
        fn get_num_events(self: &MidiBuffer) -> i32;

        /// Adds a MIDI event to the buffer at the given sample position.
        #[cxx_name = "addEvent"]
        fn add_event(self: &mut MidiBuffer, message: &MidiMessage, sample_number: i32) -> bool;

        /// Removes all events within a sample range.
        #[cxx_name = "clear"]
        fn clear_range(self: &mut MidiBuffer, start_sample: i32, num_samples: i32);

        type MidiMessage = super::MidiMessage;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn midi_message_drop(msg: &mut MidiMessage);

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn midi_message_clone(message: &MidiMessage) -> MidiMessage;

        /// Creates a note-on message.
        #[cxx_name = "noteOn"]
        #[Self = "MidiMessage"]
        fn note_on(channel: i32, note_number: i32, velocity: f32) -> MidiMessage;

        /// Creates a note-off message.
        #[cxx_name = "noteOff"]
        #[Self = "MidiMessage"]
        fn note_off(channel: i32, note_number: i32, velocity: f32) -> MidiMessage;

        /// Returns a human-readable description of the message.
        #[cxx_name = "getDescription"]
        fn get_description(self: &MidiMessage) -> JuceString;

        /// Returns `true` if this is a note-on message.
        #[cxx_name = "isNoteOn"]
        fn is_note_on(self: &MidiMessage, return_true_for_note_on_velocity_zero: bool) -> bool;

        /// Returns `true` if this is a note-off message.
        #[cxx_name = "isNoteOff"]
        fn is_note_off(self: &MidiMessage, return_true_for_note_on_velocity_zero: bool) -> bool;

        /// Returns the MIDI channel (1-16) of the message.
        #[cxx_name = "getChannel"]
        fn get_channel(self: &MidiMessage) -> i32;

        /// Returns the note number (0-127) of the message.
        #[cxx_name = "getNoteNumber"]
        fn get_note_number(self: &MidiMessage) -> i32;

        /// Returns the velocity (0-127) of the message.
        #[cxx_name = "getVelocity"]
        fn get_velocity(self: &MidiMessage) -> u8;

        /// Returns the velocity as a float (0.0-1.0).
        #[cxx_name = "getFloatVelocity"]
        fn get_float_velocity(self: &MidiMessage) -> f32;

        /// Returns the timestamp of the message.
        #[cxx_name = "getTimeStamp"]
        fn get_time_stamp(self: &MidiMessage) -> f64;

        type MidiFile = super::MidiFile;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn midi_file_drop(file: &mut MidiFile);

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn midi_file_new() -> MidiFile;

        /// Returns the number of tracks in the file.
        #[cxx_name = "getNumTracks"]
        fn get_num_tracks(self: &MidiFile) -> i32;

        /// Returns the time format of the file.
        #[cxx_name = "getTimeFormat"]
        fn get_time_format(self: &MidiFile) -> i16;

        /// Sets the time format to use ticks per quarter note.
        #[cxx_name = "setTicksPerQuarterNote"]
        fn set_ticks_per_quarter_note(self: &mut MidiFile, ticks_per_quarter_note: i32);

        /// Sets the time format to SMPTE.
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
            "\"Note on C3 Velocity 102 Channel 1\""
        );
    }
}
