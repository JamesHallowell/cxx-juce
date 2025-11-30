pub use juce::SystemStats;

#[cxx::bridge(namespace = "juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        type JuceString = crate::juce_core::JuceString;

        type SystemStats;

        #[cxx_name = "getJUCEVersion"]
        #[Self = "SystemStats"]
        fn get_juce_version() -> JuceString;

        #[cxx_name = "getStackBacktrace"]
        #[Self = "SystemStats"]
        fn get_stack_backtrace() -> JuceString;
    }
}
