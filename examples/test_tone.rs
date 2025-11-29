use {
    cxx_juce::{juce_audio_devices::AudioDeviceManager, JuceError, JUCE},
    std::{thread::sleep, time::Duration},
};

fn main() -> Result<(), JuceError> {
    let juce = JUCE::initialise();
    let mut audio_device_manager = AudioDeviceManager::new(&juce);
    audio_device_manager.initialise(0, 2)?;

    {
        let mut device = audio_device_manager
            .current_device()
            .expect("default device not found");

        println!("Name: {}", device.get_name());
        println!("Type: {}", device.get_type_name());
        println!("Sample rate: {}", device.as_mut().get_current_sample_rate());
        println!(
            "Buffer size: {}",
            device.as_mut().get_current_buffer_size_samples()
        );
        println!(
            "Available sample rates: {:?}",
            device.as_mut().get_available_sample_rates()
        );
        println!(
            "Available buffer sizes: {:?}",
            device.as_mut().get_available_buffer_sizes()
        );
    }

    audio_device_manager.play_test_sound();
    sleep(Duration::from_secs(1));

    Ok(())
}
