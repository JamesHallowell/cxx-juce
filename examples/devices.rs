use cxx_juce::{juce_audio_devices::AudioDeviceManager, JuceError, JUCE};

fn main() -> Result<(), JuceError> {
    let juce = JUCE::initialise();

    let mut audio_device_manager = AudioDeviceManager::new(&juce);
    audio_device_manager.initialise(2, 2)?;

    let device_type = audio_device_manager.current_device_type().unwrap();

    println!("Inputs:");
    for input in &device_type.get_input_device_names() {
        println!("  {}", input);
    }

    println!("Outputs:");
    for output in &device_type.get_output_device_names() {
        println!("  {}", output);
    }

    Ok(())
}
