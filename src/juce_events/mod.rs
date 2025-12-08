mod application;
mod message_manager;

pub use application::{App, AppHandle, AppTimerId, On};
pub(crate) use message_manager::MessageManager;

use crate::utils::PhantomUnsend;
use std::{
    cell::{Cell, RefCell},
    sync::atomic::{AtomicU64, Ordering},
    thread_local,
};

const INVALID_THREAD_ID: u64 = 0;
static NEXT_THREAD_ID: AtomicU64 = AtomicU64::new(1);
static JUCE_THREAD_ID: AtomicU64 = AtomicU64::new(INVALID_THREAD_ID);

thread_local! {
    static THIS_THREAD_ID: Cell<u64> = {
        Cell::new(NEXT_THREAD_ID.fetch_add(1, Ordering::SeqCst))
    };
    static JUCE_APP: RefCell<Option<JuceApp>> = const { RefCell::new(None) };
}

struct JuceApp;

impl JuceApp {
    fn new() -> Self {
        #[cfg(target_os = "macos")]
        juce::initialise_ns_application();
        juce::initialise_juce();
        Self
    }
}

impl Drop for JuceApp {
    fn drop(&mut self) {
        juce::shutdown_juce();
        JUCE_THREAD_ID.store(INVALID_THREAD_ID, Ordering::SeqCst);
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

/// A handle to the JUCE runtime. Required for certain JUCE classes.
pub struct JUCE {
    _marker: PhantomUnsend,
}

impl JUCE {
    /// Run a JUCE app.
    pub fn run_app<F, A>(setup: F)
    where
        F: FnOnce(&Self) -> A + 'static,
        A: App + 'static,
    {
        application::run_app(setup);
    }

    /// Run the JUCE event loop.
    pub fn run() {
        assert!(Self::is_this_the_message_thread());
        MessageManager::run_dispatch_loop();
    }

    /// Quit JUCE app.
    pub fn quit() {
        MessageManager::stop_dispatch_loop();
    }

    fn get() -> Self {
        JUCE {
            _marker: PhantomUnsend::new(),
        }
    }

    /// Initialises the JUCE runtime on this thread if not already initialised.
    ///
    /// # Panics
    ///
    /// This function will panic if the JUCE runtime is already initialised on another thread.
    pub fn initialise() -> Self {
        Self::try_initialise().expect("failed to initialise JUCE")
    }

    /// Returns true if this is the message thread.
    fn is_this_the_message_thread() -> bool {
        JUCE_THREAD_ID.load(Ordering::SeqCst) == THIS_THREAD_ID.get()
    }

    fn try_initialise() -> Result<Self, InitialiseError> {
        let result = JUCE_THREAD_ID.compare_exchange(
            INVALID_THREAD_ID,
            THIS_THREAD_ID.get(),
            Ordering::SeqCst,
            Ordering::SeqCst,
        );

        match result {
            Ok(_) => {
                JUCE_APP.replace(Some(JuceApp::new()));
                Ok(JUCE::get())
            }
            Err(thread_id) if thread_id == THIS_THREAD_ID.get() => Ok(JUCE::get()),
            Err(_) => Err(InitialiseError::JuceAlreadyInitialised),
        }
    }
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        #[cxx_name = "initialiseJuce_GUI"]
        pub fn initialise_juce();

        #[cxx_name = "shutdownJuce_GUI"]
        pub fn shutdown_juce();

        #[cfg(target_os = "macos")]
        #[cxx_name = "initialiseNSApplication"]
        pub fn initialise_ns_application();
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
    fn initialising_juce_twice_on_the_same_thread_should_not_panic() {
        let _juce = JUCE::initialise();
        let _juce = JUCE::initialise();
    }

    #[test]
    fn juce_cant_be_initialised_simultaneously_on_two_different_threads() {
        let _juce = JUCE::initialise();

        assert!(try_to_initialise_juce_on_new_thread().is_err());
    }

    #[test]
    fn juce_can_run_on_a_different_thread_after_finishing_on_another() {
        assert!(try_to_initialise_juce_on_new_thread().is_ok());
        assert!(try_to_initialise_juce_on_new_thread().is_ok());
    }
}
