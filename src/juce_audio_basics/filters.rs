use crate::define_juce_type;

#[cxx::bridge(namespace = "cxx_juce")]
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
        include!("cxx_juce_audio_basics/cxx_juce_audio_basics.h");

        #[namespace = "juce"]
        type IIRCoefficients = super::IIRCoefficients;

        #[rust_name = "drop_iir_coefficients"]
        fn drop(filter: &mut IIRCoefficients);

        #[namespace = "juce"]
        #[cxx_name = "makeLowPass"]
        #[Self = "IIRCoefficients"]
        /// Make a low-pass filter.
        fn make_low_pass(sample_rate: f64, frequency: f64, q: f64) -> IIRCoefficients;

        #[namespace = "juce"]
        #[cxx_name = "makeHighPass"]
        #[Self = "IIRCoefficients"]
        /// Make a high-pass filter.
        fn make_high_pass(sample_rate: f64, frequency: f64, q: f64) -> IIRCoefficients;

        #[namespace = "juce"]
        #[cxx_name = "makeNotchFilter"]
        #[Self = "IIRCoefficients"]
        /// Make a notch filter.
        fn make_notch_filter(sample_rate: f64, frequency: f64, q: f64) -> IIRCoefficients;

        #[namespace = "juce"]
        pub type SingleThreadedIIRFilter = crate::juce_audio_basics::SingleThreadedIIRFilter;

        #[rust_name = "construct_single_threaded_iir_filter"]
        fn construct() -> SingleThreadedIIRFilter;

        #[rust_name = "drop_single_threaded_iir_filter"]
        fn drop(filter: &mut SingleThreadedIIRFilter);

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

define_juce_type!(
    IIRCoefficients,
    size = 20,
    align = 4,
    check_with = juce::IIRCoefficientsLayout
);

impl Drop for IIRCoefficients {
    fn drop(&mut self) {
        juce::drop_iir_coefficients(self);
    }
}

unsafe impl cxx::ExternType for IIRCoefficients {
    type Id = cxx::type_id!("juce::IIRCoefficients");
    type Kind = cxx::kind::Trivial;
}

define_juce_type!(
    SingleThreadedIIRFilter,
    size = 36,
    align = 4,
    check_with = juce::SingleThreadedIIRFilterLayout
);

impl Drop for SingleThreadedIIRFilter {
    fn drop(&mut self) {
        juce::drop_single_threaded_iir_filter(self);
    }
}

unsafe impl cxx::ExternType for SingleThreadedIIRFilter {
    type Id = cxx::type_id!("juce::SingleThreadedIIRFilter");
    type Kind = cxx::kind::Trivial;
}

unsafe impl Send for SingleThreadedIIRFilter {}

impl Default for SingleThreadedIIRFilter {
    fn default() -> Self {
        juce::construct_single_threaded_iir_filter()
    }
}

impl SingleThreadedIIRFilter {
    /// Filter the given samples.
    pub fn process(&mut self, samples: &mut [f32]) {
        unsafe { self.process_samples(samples.as_mut_ptr(), samples.len() as i32) }
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
