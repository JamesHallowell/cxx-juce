pub use juce::MidiBuffer;

#[cxx::bridge(namespace = "juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        type MidiBuffer;
    }
}
