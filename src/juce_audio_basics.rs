//! Classes for audio buffer manipulation, midi message handling, synthesis, etc.

use {
    std::f64::consts::FRAC_1_SQRT_2,
    {crate::juce, cxx::UniquePtr},
};

/// An infinite impulse response (IIR) filter.
pub struct IIRFilter(UniquePtr<juce::SingleThreadedIIRFilter>);

unsafe impl Send for IIRFilter {}

/// The quality (Q) factor of a filter.
#[derive(Debug, Copy, Clone)]
pub struct Q(pub f64);

impl Default for Q {
    fn default() -> Self {
        Self(FRAC_1_SQRT_2)
    }
}

impl IIRFilter {
    /// Create a low-pass filter.
    pub fn low_pass(sample_rate: f64, frequency: f64, Q(q): Q) -> Self {
        Self(juce::create_iir_filter(juce::make_low_pass(
            sample_rate,
            frequency,
            q,
        )))
    }

    /// Create a high-pass filter.
    pub fn high_pass(sample_rate: f64, frequency: f64, Q(q): Q) -> Self {
        Self(juce::create_iir_filter(juce::make_high_pass(
            sample_rate,
            frequency,
            q,
        )))
    }

    /// Create a notch filter.
    pub fn notch(sample_rate: f64, frequency: f64, Q(q): Q) -> Self {
        Self(juce::create_iir_filter(juce::make_notch_filter(
            sample_rate,
            frequency,
            q,
        )))
    }

    /// Filter the given samples.
    pub fn process(&mut self, samples: &mut [f32]) {
        unsafe {
            self.0
                .pin_mut()
                .process_samples(samples.as_mut_ptr(), samples.len() as i32)
        }
    }
}
