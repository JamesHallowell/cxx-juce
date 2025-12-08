use {
    cxx_juce::{
        juce_audio_basics::AudioSampleBuffer,
        juce_audio_devices::{AudioDeviceCallback, AudioDeviceManager, AudioIODevice},
        JuceError, JUCE,
    },
    std::{iter::successors, pin::Pin, thread::sleep, time::Duration},
};

#[derive(Debug, Default, Copy, Clone)]
struct ToneGenerator {
    amplitude: f64,
    frequency: f64,
    phase: f64,
    increment: f64,
}

impl Iterator for ToneGenerator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.phase.sin() * self.amplitude;
        self.phase += self.increment;
        Some(sample)
    }
}

#[derive(Default)]
struct AudioCallback {
    tones: Vec<ToneGenerator>,
}

impl AudioDeviceCallback for AudioCallback {
    fn about_to_start(&mut self, mut device: Pin<&mut AudioIODevice>) {
        const STARTING_FREQUENCY: f64 = 1024.0;

        let sample_rate = device.as_mut().get_current_sample_rate();

        let output_channels = device
            .get_active_output_channels()
            .count_number_of_set_bits() as usize;

        self.tones = successors(
            Some(ToneGenerator {
                amplitude: 0.25,
                frequency: STARTING_FREQUENCY,
                phase: 0.0,
                increment: STARTING_FREQUENCY / sample_rate,
            }),
            |prev| {
                let frequency = prev.frequency * 2.5;
                Some(ToneGenerator {
                    frequency,
                    increment: frequency / sample_rate,
                    ..*prev
                })
            },
        )
        .take(output_channels)
        .collect();
    }

    fn process_block(
        &mut self,
        _input: &AudioSampleBuffer,
        mut output: Pin<&mut AudioSampleBuffer>,
    ) {
        for channel in 0..output.get_num_channels() {
            let samples = output.as_mut().get_write_slice(channel);
            let tone = &mut self.tones[channel as usize];

            for (sample, tone) in samples.iter_mut().zip(tone) {
                *sample = tone as f32;
            }
        }
    }

    fn stopped(&mut self) {}
}

fn main() -> Result<(), JuceError> {
    let juce = JUCE::initialise();

    let mut device_manager = AudioDeviceManager::new(&juce);
    device_manager.initialise(0, 2)?;

    let handle = device_manager.add_audio_callback(AudioCallback::default());

    sleep(Duration::from_secs(2));

    device_manager.remove_audio_callback(handle);

    Ok(())
}
