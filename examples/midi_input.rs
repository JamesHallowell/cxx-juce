use cxx_juce::{juce_audio_basics::MidiMessage, juce_audio_devices::MidiInput, JUCE};

fn main() {
    let _juce = JUCE::initialise();

    let devices = MidiInput::get_available_devices();
    if devices.is_empty() {
        println!("No MIDI input devices found");
        return;
    }

    let default_device = MidiInput::get_default_device();

    let on_midi_message = |message: &MidiMessage| {
        println!("[{:.3}s] {message:?}", message.get_time_stamp());
    };

    let mut input = match MidiInput::open(&default_device.identifier, on_midi_message) {
        Some(input) => {
            println!("Input device: {:?}", input.get_device_info());
            input
        }
        None => {
            println!("Failed to open device");
            return;
        }
    };

    input.pin_mut().start();

    std::thread::sleep(std::time::Duration::from_secs(10));

    input.pin_mut().stop();
}
