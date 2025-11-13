//! Core JUCE types.

mod array;
mod bigint;
mod string;
mod system;

pub use {
    array::{DoubleArray, IntArray, StringArray},
    bigint::BigInteger,
    string::JuceString,
    system::SystemStats,
};
