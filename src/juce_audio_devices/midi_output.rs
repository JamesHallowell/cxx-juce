pub use juce::MidiOutput;

#[cxx::bridge(namespace = "juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        type JuceString = crate::juce_core::JuceString;
        type MidiMessage = crate::juce_audio_basics::MidiMessage;
        type MidiBuffer = crate::juce_audio_basics::MidiBuffer;
        type MidiDeviceInfo = crate::juce_audio_devices::MidiDeviceInfo;
        type MidiDeviceInfoArray = crate::juce_audio_devices::MidiDeviceInfoArray;
        /// A MIDI output device.
        pub type MidiOutput;

        /// Returns the available MIDI output devices.
        #[Self = "MidiOutput"]
        #[cxx_name = "getAvailableDevices"]
        fn get_available_devices() -> MidiDeviceInfoArray;

        /// Returns the default MIDI output device.
        #[Self = "MidiOutput"]
        #[cxx_name = "getDefaultDevice"]
        fn get_default_device() -> MidiDeviceInfo;

        /// Opens a MIDI output device with the given identifier.
        #[Self = "MidiOutput"]
        #[cxx_name = "openDevice"]
        fn open_device(device_identifier: &JuceString) -> UniquePtr<MidiOutput>;

        /// Returns the device info for this output.
        #[cxx_name = "getDeviceInfo"]
        fn get_device_info(self: &MidiOutput) -> MidiDeviceInfo;

        /// Sets the name of this output device.
        #[cxx_name = "setName"]
        fn set_name(self: Pin<&mut MidiOutput>, new_name: &JuceString);

        /// Sends a MIDI message immediately.
        #[cxx_name = "sendMessageNow"]
        fn send_message_now(self: Pin<&mut MidiOutput>, message: &MidiMessage);

        /// Sends a block of MIDI messages immediately.
        #[cxx_name = "sendBlockOfMessagesNow"]
        fn send_block_of_messages_now(self: Pin<&mut MidiOutput>, buffer: &MidiBuffer);

        /// Sends a block of MIDI messages, timestamped for playback.
        #[cxx_name = "sendBlockOfMessages"]
        fn send_block_of_messages(
            self: Pin<&mut MidiOutput>,
            buffer: &MidiBuffer,
            millisecond_counter_to_start_at: f64,
            samples_per_second_for_buffer: f64,
        );

        /// Clears all pending messages from the output queue.
        #[cxx_name = "clearAllPendingMessages"]
        fn clear_all_pending_messages(self: Pin<&mut MidiOutput>);

        /// Starts the background thread for timed message delivery.
        #[cxx_name = "startBackgroundThread"]
        fn start_background_thread(self: Pin<&mut MidiOutput>);

        /// Stops the background thread.
        #[cxx_name = "stopBackgroundThread"]
        fn stop_background_thread(self: Pin<&mut MidiOutput>);

        /// Returns `true` if the background thread is running.
        #[cxx_name = "isBackgroundThreadRunning"]
        fn is_background_thread_running(self: &MidiOutput) -> bool;
    }
}
