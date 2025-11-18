//! Core JUCE types.

mod array;
mod bigint;
mod file;
mod string;
mod system;
mod time;

pub use {
    array::{DoubleArray, IntArray, StringArray},
    bigint::BigInteger,
    file::{File, FileSearchPath},
    string::JuceString,
    system::SystemStats,
    time::Time,
};

pub(crate) use array::ArrayLayout;
