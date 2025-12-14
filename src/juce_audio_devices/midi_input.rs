use crate::define_trait;
use cxx::UniquePtr;

pub use juce::MidiInput;

pub struct MidiInputWithCallback {
    device: UniquePtr<MidiInput>,
    _callback: UniquePtr<juce::MidiInputCallback>,
}

impl std::ops::Deref for MidiInputWithCallback {
    type Target = UniquePtr<MidiInput>;
    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl std::ops::DerefMut for MidiInputWithCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.device
    }
}

impl MidiInput {
    pub fn open(
        device: &juce::JuceString,
        callback: impl FnMut(&juce::MidiMessage) + Send + 'static,
    ) -> Option<MidiInputWithCallback> {
        let callback = wrap_midi_input_callback(callback);
        let device = unsafe { Self::open_device(device, callback.as_mut_ptr()) };
        (!device.is_null()).then(|| MidiInputWithCallback {
            device,
            _callback: callback,
        })
    }
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        type JuceString = crate::juce_core::JuceString;
        type MidiDeviceInfo = crate::juce_audio_devices::MidiDeviceInfo;
        type MidiDeviceInfoArray = crate::juce_audio_devices::MidiDeviceInfoArray;
        type MidiMessage = crate::juce_audio_basics::MidiMessage;
        type MidiInput;
        type MidiInputCallback;

        #[namespace = "cxx_juce"]
        type BoxDynMidiInputCallback = Box<dyn super::MidiInputCallback>;

        #[namespace = "cxx_juce"]
        #[cxx_name = "wrap"]
        fn wrap_midi_input_callback(
            callback: BoxDynMidiInputCallback,
        ) -> UniquePtr<MidiInputCallback>;

        #[cxx_name = "getAvailableDevices"]
        #[Self = "MidiInput"]
        fn get_available_devices() -> MidiDeviceInfoArray;

        #[Self = "MidiInput"]
        #[cxx_name = "getDefaultDevice"]
        fn get_default_device() -> MidiDeviceInfo;

        #[Self = "MidiInput"]
        #[cxx_name = "openDevice"]
        unsafe fn open_device(
            device_identifier: &JuceString,
            callback: *mut MidiInputCallback,
        ) -> UniquePtr<MidiInput>;

        #[cxx_name = "getDeviceInfo"]
        fn get_device_info(self: &MidiInput) -> MidiDeviceInfo;

        #[cxx_name = "start"]
        fn start(self: Pin<&mut MidiInput>);

        #[cxx_name = "stop"]
        fn stop(self: Pin<&mut MidiInput>);
    }

    #[namespace = "cxx_juce"]
    extern "Rust" {
        type MidiInputCallbackImpl;

        #[Self = "MidiInputCallbackImpl"]
        unsafe fn drop(callback: *mut BoxDynMidiInputCallback);

        #[Self = "MidiInputCallbackImpl"]
        unsafe fn handle_incoming_midi_message(
            callback: &mut BoxDynMidiInputCallback,
            message: &MidiMessage,
        );
    }
}

fn wrap_midi_input_callback(
    callback: impl FnMut(&juce::MidiMessage) + Send + 'static,
) -> UniquePtr<juce::MidiInputCallback> {
    struct Wrapper<Callback>(Callback);

    impl<Callback> MidiInputCallback for Wrapper<Callback>
    where
        Callback: FnMut(&juce::MidiMessage) + Send + 'static,
    {
        fn handle_incoming_midi_message(&mut self, message: &juce::MidiMessage) {
            self.0(message);
        }
    }

    juce::wrap_midi_input_callback(Box::new(Wrapper(callback)))
}

define_trait! {
    MidiInputCallback: Send,
    MidiInputCallbackImpl,
    "cxx_juce::BoxDynMidiInputCallback",
    fn handle_incoming_midi_message(&mut self, message: &juce::MidiMessage);
}
