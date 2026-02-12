pub use juce::SystemStats;

#[cxx::bridge(namespace = "juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        type JuceString = crate::juce_core::JuceString;

        /// System statistics and information.
        type SystemStats;

        /// Returns the JUCE version string.
        #[cxx_name = "getJUCEVersion"]
        #[Self = "SystemStats"]
        fn get_juce_version() -> JuceString;

        /// Returns the current stack backtrace as a string.
        #[cxx_name = "getStackBacktrace"]
        #[Self = "SystemStats"]
        fn get_stack_backtrace() -> JuceString;
    }
}
