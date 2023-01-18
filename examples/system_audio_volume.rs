use {
    cxx_juce::juce_audio_devices::SystemAudioVolume,
    std::{thread::sleep, time::Duration},
};

fn delay() {
    sleep(Duration::from_secs(1));
}

fn main() {
    let original_gain = SystemAudioVolume::get_gain();
    println!("System gain is currently set at {original_gain}");

    delay();

    println!("Halving the gain 🤫");
    SystemAudioVolume::set_gain(original_gain / 2.0);

    delay();

    println!("Muting 🔇");
    SystemAudioVolume::mute();

    delay();

    println!("Unmuting 🔊");
    SystemAudioVolume::unmute();

    delay();

    print!("Putting it back to how you had it 🧹");
    SystemAudioVolume::set_gain(original_gain);
}
