use cxx_juce::{
    juce_audio_devices::{AudioDeviceManager, AudioIODeviceType},
    Result,
};

fn main() -> Result<()> {
    let mut audio_device_manager = AudioDeviceManager::new();
    audio_device_manager.initialise(2, 2)?;

    let device_type = audio_device_manager.current_device_type().unwrap();

    println!("Inputs:");
    for input in device_type.input_devices() {
        println!("  {}", input);
    }

    println!("Outputs:");
    for output in device_type.output_devices() {
        println!("  {}", output);
    }

    Ok(())
}
