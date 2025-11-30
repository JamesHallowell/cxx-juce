pub use juce::MidiBuffer;

#[cxx::bridge(namespace = "juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");
        include!("cxx_juce_audio_basics/cxx_juce_audio_basics.h");

        type MidiBuffer;
    }
}
