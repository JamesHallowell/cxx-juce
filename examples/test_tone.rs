use {
    cxx_juce::{
        juce_audio_devices::{AudioDeviceManager, AudioIODevice},
        Result, JUCE,
    },
    std::{thread::sleep, time::Duration},
};

fn main() -> Result<()> {
    let juce = JUCE::initialise();
    let mut audio_device_manager = AudioDeviceManager::new(&juce);
    audio_device_manager.initialise(0, 2)?;

    {
        let mut device = audio_device_manager
            .current_device()
            .expect("default device not found");

        println!("Name: {}", device.name());
        println!("Type: {}", device.type_name());
        println!("Sample rate: {}", device.sample_rate());
        println!("Buffer size: {}", device.buffer_size());
        println!(
            "Available sample rates: {:?}",
            device.available_sample_rates()
        );
        println!(
            "Available buffer sizes: {:?}",
            device.available_buffer_sizes()
        );
    }

    audio_device_manager.play_test_sound();
    sleep(Duration::from_secs(1));

    Ok(())
}
