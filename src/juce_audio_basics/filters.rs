use crate::define_juce_type;

define_juce_type! {
    IIRCoefficients,
    layout = juce::IIRCoefficientsLayout,
    cxx_name = "juce::IIRCoefficients",
    drop = juce::iir_coefficients_drop,
}

define_juce_type! {
    SingleThreadedIIRFilter,
    layout = juce::SingleThreadedIIRFilterLayout,
    cxx_name = "juce::SingleThreadedIIRFilter",
    drop = juce::single_threaded_iir_filter_drop,
    default = juce::single_threaded_iir_filter_new,
}

unsafe impl Send for SingleThreadedIIRFilter {}

impl SingleThreadedIIRFilter {
    /// Filter the given samples.
    pub fn process(&mut self, samples: &mut [f32]) {
        unsafe { self.process_samples(samples.as_mut_ptr(), samples.len() as i32) }
    }
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum IIRCoefficientsLayout {
        Size = 20,
        Alignment = 4,
    }

    enum SingleThreadedIIRFilterLayout {
        Size = 36,
        Alignment = 4,
    }

    unsafe extern "C++" {
        include!("cxx_juce.h");

        type IIRCoefficients = super::IIRCoefficients;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn iir_coefficients_drop(filter: &mut IIRCoefficients);

        #[cxx_name = "makeLowPass"]
        #[Self = "IIRCoefficients"]
        /// Make a low-pass filter.
        fn make_low_pass(sample_rate: f64, frequency: f64, q: f64) -> IIRCoefficients;

        #[cxx_name = "makeHighPass"]
        #[Self = "IIRCoefficients"]
        /// Make a high-pass filter.
        fn make_high_pass(sample_rate: f64, frequency: f64, q: f64) -> IIRCoefficients;

        #[cxx_name = "makeNotchFilter"]
        #[Self = "IIRCoefficients"]
        /// Make a notch filter.
        fn make_notch_filter(sample_rate: f64, frequency: f64, q: f64) -> IIRCoefficients;

        pub type SingleThreadedIIRFilter = crate::juce_audio_basics::SingleThreadedIIRFilter;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn single_threaded_iir_filter_new() -> SingleThreadedIIRFilter;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn single_threaded_iir_filter_drop(self_: &mut SingleThreadedIIRFilter);

        #[cxx_name = "setCoefficients"]
        /// Applies a set of coefficients to this filter.
        fn set_coefficients(self: &mut SingleThreadedIIRFilter, coefficients: &IIRCoefficients);

        #[cxx_name = "processSamples"]
        unsafe fn process_samples(
            self: &mut SingleThreadedIIRFilter,
            samples: *mut f32,
            num_samples: i32,
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creating_a_low_pass_filter() {
        let mut filter = SingleThreadedIIRFilter::default();

        let coefficients = IIRCoefficients::make_low_pass(48000.0, 2000.0, 0.7);
        filter.set_coefficients(&coefficients);

        let mut samples = vec![0_f32; 512];
        filter.process(&mut samples);
    }
}
