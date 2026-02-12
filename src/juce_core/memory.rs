pub use juce::MemoryBlock;

#[cxx::bridge(namespace = "juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        /// A block of memory.
        type MemoryBlock;
    }
}
