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
        pub type MidiOutput;

        #[Self = "MidiOutput"]
        #[cxx_name = "getAvailableDevices"]
        fn get_available_devices() -> MidiDeviceInfoArray;

        #[Self = "MidiOutput"]
        #[cxx_name = "getDefaultDevice"]
        fn get_default_device() -> MidiDeviceInfo;

        #[Self = "MidiOutput"]
        #[cxx_name = "openDevice"]
        fn open_device(device_identifier: &JuceString) -> UniquePtr<MidiOutput>;

        #[cxx_name = "getDeviceInfo"]
        fn get_device_info(self: &MidiOutput) -> MidiDeviceInfo;

        #[cxx_name = "setName"]
        fn set_name(self: Pin<&mut MidiOutput>, new_name: &JuceString);

        #[cxx_name = "sendMessageNow"]
        fn send_message_now(self: Pin<&mut MidiOutput>, message: &MidiMessage);

        #[cxx_name = "sendBlockOfMessagesNow"]
        fn send_block_of_messages_now(self: Pin<&mut MidiOutput>, buffer: &MidiBuffer);

        #[cxx_name = "sendBlockOfMessages"]
        fn send_block_of_messages(
            self: Pin<&mut MidiOutput>,
            buffer: &MidiBuffer,
            millisecond_counter_to_start_at: f64,
            samples_per_second_for_buffer: f64,
        );

        #[cxx_name = "clearAllPendingMessages"]
        fn clear_all_pending_messages(self: Pin<&mut MidiOutput>);

        #[cxx_name = "startBackgroundThread"]
        fn start_background_thread(self: Pin<&mut MidiOutput>);

        #[cxx_name = "stopBackgroundThread"]
        fn stop_background_thread(self: Pin<&mut MidiOutput>);

        #[cxx_name = "isBackgroundThreadRunning"]
        fn is_background_thread_running(self: &MidiOutput) -> bool;
    }
}
