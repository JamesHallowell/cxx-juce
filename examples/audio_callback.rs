use {
    cxx_juce::{
        juce_audio_devices::{
            AudioDeviceManager, AudioIODeviceCallback, InputAudioSampleBuffer,
            OutputAudioSampleBuffer,
        },
        Result,
    },
    std::{iter::successors, thread::sleep, time::Duration},
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

impl AudioIODeviceCallback for AudioCallback {
    fn about_to_start(&mut self, _: usize, output_channels: usize, sample_rate: f64, _: usize) {
        const STARTING_FREQUENCY: f64 = 1024.0;

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
        _input: &InputAudioSampleBuffer<'_>,
        output: &mut OutputAudioSampleBuffer<'_>,
    ) {
        for channel in 0..output.channels() {
            let samples = &mut output[channel];
            let tone = &mut self.tones[channel];

            for (sample, tone) in samples.iter_mut().zip(tone) {
                *sample = tone as f32;
            }
        }
    }

    fn stopped(&mut self) {}
}

fn main() -> Result<()> {
    let mut device_manager = AudioDeviceManager::new();
    device_manager.initialise(0, 2)?;

    let _handle = device_manager.add_audio_callback(AudioCallback::default());

    sleep(Duration::from_secs(2));

    Ok(())
}
