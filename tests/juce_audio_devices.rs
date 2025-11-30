use cxx::UniquePtr;
use cxx_juce::{
    juce_audio_devices::{
        AudioDevice, AudioDeviceManager, AudioDeviceSetup, AudioDeviceType, AudioIODevice,
        ChannelCount,
    },
    juce_core::JuceString,
    JUCE,
};

#[derive(Default)]
struct MockAudioDeviceType {
    input_devices: Vec<String>,
    output_devices: Vec<String>,
}

impl AudioDeviceType for MockAudioDeviceType {
    fn name(&self) -> String {
        "Test".to_string()
    }

    fn scan_for_devices(&mut self) {
        self.input_devices = ["Microphone", "Audio Interface", "Headset"]
            .into_iter()
            .map(String::from)
            .collect();

        self.output_devices = ["Speakers", "Headphones"]
            .into_iter()
            .map(String::from)
            .collect();
    }

    fn input_devices(&self) -> Vec<String> {
        self.input_devices.clone()
    }

    fn output_devices(&self) -> Vec<String> {
        self.output_devices.clone()
    }

    fn create_device(
        &mut self,
        input_device_name: &JuceString,
        output_device_name: &JuceString,
    ) -> UniquePtr<AudioIODevice> {
        let device: Box<dyn AudioDevice> = Box::new(MockAudioDevice {
            name: format!("{} / {}", input_device_name, output_device_name),
            type_name: self.name(),
            sample_rate: 44100.0,
            buffer_size: 128,
        });

        device.into()
    }
}

struct MockAudioDevice {
    name: String,
    type_name: String,
    sample_rate: f64,
    buffer_size: i32,
}

impl AudioDevice for MockAudioDevice {
    fn name(&self) -> &str {
        &self.name
    }

    fn type_name(&self) -> &str {
        &self.type_name
    }

    fn sample_rate(&mut self) -> f64 {
        self.sample_rate
    }

    fn buffer_size(&mut self) -> i32 {
        self.buffer_size
    }

    fn available_sample_rates(&mut self) -> Vec<f64> {
        vec![44100.0, 48000.0]
    }

    fn available_buffer_sizes(&mut self) -> Vec<i32> {
        vec![128, 256, 512]
    }

    fn open(&mut self, sample_rate: f64, buffer_size: i32) -> JuceString {
        self.sample_rate = sample_rate;
        self.buffer_size = buffer_size;
        JuceString::default()
    }

    fn close(&mut self) {}

    fn input_channels(&self) -> i32 {
        2
    }

    fn output_channels(&self) -> i32 {
        2
    }
}

#[test]
fn can_query_audio_device_types() {
    let juce = JUCE::initialise();
    let mut audio_device_manager = AudioDeviceManager::new(&juce);
    audio_device_manager.add_audio_device_type(MockAudioDeviceType::default());
    audio_device_manager.set_current_audio_device_type("Test", true);

    let mut device_type = audio_device_manager.current_device_type().unwrap();

    assert_eq!(device_type.get_type_name(), "Test");

    device_type.as_mut().scan_for_devices();

    assert_eq!(
        device_type
            .as_mut()
            .get_input_device_names()
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
        ["Microphone", "Audio Interface", "Headset"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>()
    );

    assert_eq!(
        device_type
            .get_output_device_names()
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
        ["Speakers", "Headphones"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>()
    );
}

#[test]
fn can_configure_audio_device_setup() {
    let juce = JUCE::initialise();
    let mut audio_device_manager = AudioDeviceManager::new(&juce);
    audio_device_manager.add_audio_device_type(MockAudioDeviceType::default());
    audio_device_manager.set_current_audio_device_type("Test", true);
    audio_device_manager
        .current_device_type()
        .unwrap()
        .scan_for_devices();

    let setup = AudioDeviceSetup::default()
        .with_buffer_size(512)
        .with_sample_rate(48000.0)
        .with_input_device_name("Microphone")
        .with_output_device_name("Speakers");

    audio_device_manager
        .set_audio_device_setup(&setup, true)
        .unwrap();

    let current_setup = audio_device_manager.audio_device_setup();

    assert_eq!(current_setup.buffer_size, 512);
    assert_eq!(current_setup.sample_rate, 48000.0);
    assert_eq!(current_setup.input_device_name, "Microphone");
    assert_eq!(current_setup.output_device_name, "Speakers");
    assert_eq!(current_setup.input_channels(), ChannelCount::Default);
    assert_eq!(current_setup.output_channels(), ChannelCount::Default);
}

#[test]
fn can_create_devices() {
    let juce = JUCE::initialise();
    let mut audio_device_manager = AudioDeviceManager::new(&juce);
    audio_device_manager.add_audio_device_type(MockAudioDeviceType::default());
    audio_device_manager.set_current_audio_device_type("Test", true);
    audio_device_manager
        .current_device_type()
        .unwrap()
        .scan_for_devices();

    let device = audio_device_manager
        .current_device_type()
        .unwrap()
        .create_device("Microphone", "Speakers")
        .expect("failed to create device");

    assert_eq!(device.get_name(), "Microphone / Speakers");
    assert_eq!(device.get_type_name(), "Test");
}

#[test]
fn can_configure_channel_count_in_audio_device_setup() {
    let setup = AudioDeviceSetup::default()
        .with_input_channels(ChannelCount::Custom(4))
        .with_output_channels(ChannelCount::Default);

    assert_eq!(setup.input_channels(), ChannelCount::Custom(4));
    assert_eq!(setup.output_channels(), ChannelCount::Default);
}
