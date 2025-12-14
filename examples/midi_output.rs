use cxx_juce::{juce_audio_basics::MidiMessage, juce_audio_devices::MidiOutput, JUCE};

fn main() {
    let _juce = JUCE::initialise();

    let devices = MidiOutput::get_available_devices();
    if devices.is_empty() {
        println!("No MIDI output devices found");
        return;
    }

    let default_device = MidiOutput::get_default_device();

    let mut output = MidiOutput::open_device(&default_device.identifier);
    if !output.is_null() {
        println!("Output device: {:?}", output.get_device_info());
    } else {
        println!("Failed to open device");
        return;
    };

    const NOTES: [i32; 3] = [60, 64, 67];
    const CHANNEL: i32 = 1;
    const VELOCITY: f32 = 0.8;

    for note in NOTES {
        let message = MidiMessage::note_on(CHANNEL, note, VELOCITY);
        output.pin_mut().send_message_now(&message);
        println!("{message:?}");
    }

    std::thread::sleep(std::time::Duration::from_secs(1));

    for note in NOTES {
        let message = MidiMessage::note_off(CHANNEL, note, 0.0);
        output.pin_mut().send_message_now(&message);
        println!("{message:?}");
    }
}
