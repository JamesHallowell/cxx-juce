//! Rust bindings for [JUCE](https://juce.com/) using [cxx](https://github.com/dtolnay/cxx).

pub mod juce_audio_basics;
pub mod juce_audio_devices;
pub mod juce_audio_processors;
pub mod juce_core;
mod utils;

use crate::juce_core::JuceString;
use std::{
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
};

/// Returns the version of the JUCE library.
pub fn juce_version() -> String {
    juce_core::SystemStats::get_juce_version().into()
}

/// A handle to the JUCE runtime. Required for certain JUCE classes.
///
/// Once all references to this object are dropped, the JUCE runtime will be shut down.
#[must_use]
#[derive(Clone)]
pub struct JUCE {
    _app: Rc<JuceApp>,
}

static IS_JUCE_RUNNING: AtomicBool = AtomicBool::new(false);

struct JuceApp;

impl JuceApp {
    fn new() -> Self {
        juce::initialise_juce();

        #[cfg(target_os = "macos")]
        juce::initialise_ns_application();

        Self
    }
}

impl Drop for JuceApp {
    fn drop(&mut self) {
        juce::shutdown_juce();

        IS_JUCE_RUNNING.store(false, Ordering::SeqCst);
    }
}

#[derive(Debug)]
enum InitialiseError {
    JuceAlreadyInitialised,
}

impl std::error::Error for InitialiseError {}

impl std::fmt::Display for InitialiseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::JuceAlreadyInitialised => write!(f, "JUCE has already been initialised"),
        }
    }
}

impl JUCE {
    /// Initialises the JUCE runtime.
    ///
    /// # Panics
    ///
    /// This function will panic if the JUCE runtime is already initialised.
    pub fn initialise() -> Self {
        Self::try_initialise().unwrap()
    }

    fn try_initialise() -> Result<Self, InitialiseError> {
        let result =
            IS_JUCE_RUNNING.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst);

        if result.is_err() {
            return Err(InitialiseError::JuceAlreadyInitialised);
        }

        Ok(Self {
            _app: Rc::new(JuceApp::new()),
        })
    }
}

#[derive(Debug)]
pub struct JuceError(JuceString);

impl std::fmt::Display for JuceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.0.as_ref())
    }
}

impl std::error::Error for JuceError {}

#[cxx::bridge(namespace = "juce")]
pub(crate) mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        #[rust_name = "initialise_juce"]
        pub fn initialiseJuce_GUI();

        #[rust_name = "shutdown_juce"]
        pub fn shutdownJuce_GUI();

        #[cfg(target_os = "macos")]
        #[rust_name = "initialise_ns_application"]
        pub fn initialiseNSApplication();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn try_to_initialise_juce_on_new_thread() -> std::thread::Result<()> {
        std::thread::spawn(move || {
            let _juce = JUCE::initialise();
        })
        .join()
    }

    #[test]
    #[should_panic]
    fn initialising_juce_twice_on_the_same_thread_should_panic() {
        let _juce = JUCE::initialise();
        let _juce = JUCE::initialise();
    }

    #[test]
    fn initialising_juce_again_on_the_same_thread_after_shutdown_is_ok() {
        let juce = JUCE::initialise();
        drop(juce);

        let _juce = JUCE::initialise();
    }

    #[test]
    fn juce_cant_be_initialised_simultaneously_on_two_different_threads() {
        let _juce = JUCE::initialise();

        assert!(try_to_initialise_juce_on_new_thread().is_err());
    }

    #[test]
    fn juce_can_run_on_a_different_thread_after_finishing_on_another() {
        let juce = JUCE::initialise();
        drop(juce);

        assert!(try_to_initialise_juce_on_new_thread().is_ok());
    }

    #[test]
    fn juce_is_shutdown_once_all_references_have_been_dropped() {
        let a = JUCE::initialise();
        let b = a.clone();

        drop(a);

        assert!(try_to_initialise_juce_on_new_thread().is_err());

        drop(b);

        assert!(try_to_initialise_juce_on_new_thread().is_ok());
    }
}
