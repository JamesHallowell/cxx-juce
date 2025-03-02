//! Rust bindings for [JUCE](https://juce.com/) using [cxx](https://github.com/dtolnay/cxx).

pub mod juce_audio_basics;
pub mod juce_audio_devices;

use {
    juce_audio_devices::{
        ffi::{
            audio_io_device::{
                device_available_buffer_sizes, device_available_sample_rates, device_buffer_size,
                device_close, device_name, device_open, device_sample_rate, device_type_name,
            },
            audio_io_device_callback::{about_to_start, process_block, stopped},
            audio_io_device_type::{
                create_device, destroy_device, get_device_names, name, scan_for_devices,
            },
        },
        BoxedAudioIODevice, BoxedAudioIODeviceCallback, BoxedAudioIODeviceType,
    },
    std::{
        rc::Rc,
        sync::atomic::{AtomicBool, Ordering},
    },
};

/// Returns the version of the JUCE library.
pub fn juce_version() -> String {
    juce::version()
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

    fn try_initialise() -> std::result::Result<Self, InitialiseError> {
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

pub type Exception = cxx::Exception;
pub type Result<T> = std::result::Result<T, Exception>;

#[cxx::bridge(namespace = "cxx_juce")]
pub(crate) mod juce {
    extern "Rust" {
        type BoxedAudioIODeviceCallback;

        #[namespace = "audio_io_device_callback"]
        #[cxx_name = "aboutToStart"]
        fn about_to_start(
            callback: Pin<&mut BoxedAudioIODeviceCallback>,
            device: Pin<&mut AudioIODevice>,
        );

        #[namespace = "audio_io_device_callback"]
        #[cxx_name = "processBlock"]
        fn process_block(
            callback: Pin<&mut BoxedAudioIODeviceCallback>,
            input: &AudioSampleBuffer,
            output: Pin<&mut AudioSampleBuffer>,
        );

        #[namespace = "audio_io_device_callback"]
        #[cxx_name = "stopped"]
        fn stopped(callback: Pin<&mut BoxedAudioIODeviceCallback>);

        type BoxedAudioIODeviceType;

        #[namespace = "audio_io_device_type"]
        #[cxx_name = "name"]
        fn name(self_: &BoxedAudioIODeviceType) -> String;

        #[namespace = "audio_io_device_type"]
        #[cxx_name = "scanForDevices"]
        fn scan_for_devices(self_: Pin<&mut BoxedAudioIODeviceType>);

        #[namespace = "audio_io_device_type"]
        #[cxx_name = "getDeviceNames"]
        fn get_device_names(self_: &BoxedAudioIODeviceType, input: bool) -> Vec<String>;

        #[namespace = "audio_io_device_type"]
        #[cxx_name = "createDevice"]
        fn create_device(
            self_: Pin<&mut BoxedAudioIODeviceType>,
            input_device_name: &str,
            output_device_name: &str,
        ) -> *mut BoxedAudioIODevice;

        #[namespace = "audio_io_device_type"]
        #[cxx_name = "destroyDevice"]
        unsafe fn destroy_device(self_: *mut BoxedAudioIODevice);

        type BoxedAudioIODevice;

        #[namespace = "audio_io_device"]
        #[cxx_name = "deviceName"]
        pub fn device_name(self_: &BoxedAudioIODevice) -> String;

        #[namespace = "audio_io_device"]
        #[cxx_name = "typeName"]
        pub fn device_type_name(self_: &BoxedAudioIODevice) -> String;

        #[namespace = "audio_io_device"]
        #[cxx_name = "sampleRate"]
        pub fn device_sample_rate(self_: Pin<&mut BoxedAudioIODevice>) -> f64;

        #[namespace = "audio_io_device"]
        #[cxx_name = "bufferSize"]
        pub fn device_buffer_size(self_: Pin<&mut BoxedAudioIODevice>) -> usize;

        #[namespace = "audio_io_device"]
        #[cxx_name = "availableSampleRates"]
        pub fn device_available_sample_rates(self_: Pin<&mut BoxedAudioIODevice>) -> Vec<f64>;

        #[namespace = "audio_io_device"]
        #[cxx_name = "availableBufferSizes"]
        pub fn device_available_buffer_sizes(self_: Pin<&mut BoxedAudioIODevice>) -> Vec<usize>;

        #[namespace = "audio_io_device"]
        #[cxx_name = "open"]
        pub fn device_open(
            self_: Pin<&mut BoxedAudioIODevice>,
            sample_rate: f64,
            buffer_size: usize,
        ) -> String;

        #[namespace = "audio_io_device"]
        #[cxx_name = "close"]
        pub fn device_close(self_: Pin<&mut BoxedAudioIODevice>);
    }

    unsafe extern "C++" {
        include!("cxx-juce/bridge/cxx_juce.h");

        #[rust_name = "version"]
        pub fn juceVersion() -> String;

        #[rust_name = "initialise_juce"]
        pub fn initialiseJuce();

        #[rust_name = "shutdown_juce"]
        pub fn shutdownJuce();

        #[cfg(target_os = "macos")]
        #[namespace = "juce"]
        #[rust_name = "initialise_ns_application"]
        pub fn initialiseNSApplication();

        #[namespace = "juce"]
        pub type AudioIODeviceTypeArray;

        pub fn size(self: &AudioIODeviceTypeArray) -> i32;

        #[rust_name = "get_unchecked"]
        pub fn getUnchecked(self: &AudioIODeviceTypeArray, index: i32) -> *mut AudioIODeviceType;

        pub type AudioDeviceSetup;

        #[rust_name = "create_audio_device_setup"]
        pub fn createAudioDeviceSetup() -> UniquePtr<AudioDeviceSetup>;

        #[rust_name = "output_device_name"]
        pub fn outputDeviceName(self: &AudioDeviceSetup) -> &str;

        #[rust_name = "input_device_name"]
        pub fn inputDeviceName(self: &AudioDeviceSetup) -> &str;

        #[rust_name = "sample_rate"]
        pub fn sampleRate(self: &AudioDeviceSetup) -> f64;

        #[rust_name = "buffer_size"]
        pub fn bufferSize(self: &AudioDeviceSetup) -> i32;

        #[rust_name = "set_output_device_name"]
        pub fn setOutputDeviceName(self: Pin<&mut AudioDeviceSetup>, name: &str);

        #[rust_name = "set_input_device_name"]
        pub fn setInputDeviceName(self: Pin<&mut AudioDeviceSetup>, name: &str);

        #[rust_name = "set_sample_rate"]
        pub fn setSampleRate(self: Pin<&mut AudioDeviceSetup>, sample_rate: f64);

        #[rust_name = "set_buffer_size"]
        pub fn setBufferSize(self: Pin<&mut AudioDeviceSetup>, buffer_size: i32);

        #[rust_name = "number_of_input_channels"]
        pub fn numberOfInputChannels(self: &AudioDeviceSetup) -> i32;

        #[rust_name = "set_number_of_input_channels"]
        pub fn setNumberOfInputChannels(
            self: Pin<&mut AudioDeviceSetup>,
            number_of_input_channels: i32,
        );

        #[rust_name = "use_default_input_channels"]
        pub fn useDefaultInputChannels(self: Pin<&mut AudioDeviceSetup>, use_default: bool);

        #[rust_name = "using_default_input_channels"]
        pub fn usingDefaultInputChannels(self: &AudioDeviceSetup) -> bool;

        #[rust_name = "number_of_output_channels"]
        pub fn numberOfOutputChannels(self: &AudioDeviceSetup) -> i32;

        #[rust_name = "set_number_of_output_channels"]
        pub fn setNumberOfOutputChannels(
            self: Pin<&mut AudioDeviceSetup>,
            number_of_output_channels: i32,
        );

        #[rust_name = "use_default_output_channels"]
        pub fn useDefaultOutputChannels(self: Pin<&mut AudioDeviceSetup>, use_default: bool);

        #[rust_name = "using_default_output_channels"]
        pub fn usingDefaultOutputChannels(self: &AudioDeviceSetup) -> bool;

        pub type AudioDeviceManager;

        #[rust_name = "create_audio_device_manager"]
        pub fn createAudioDeviceManager() -> UniquePtr<AudioDeviceManager>;

        #[rust_name = "wrap_audio_callback"]
        pub fn wrapAudioCallback(
            callback: Box<BoxedAudioIODeviceCallback>,
        ) -> UniquePtr<AudioCallbackWrapper>;

        #[rust_name = "initialise_with_default_devices"]
        pub fn initialiseWithDefaultDevices(
            self: Pin<&mut AudioDeviceManager>,
            num_input_channels: i32,
            num_output_channels: i32,
        ) -> Result<()>;

        #[rust_name = "get_audio_device_setup"]
        pub fn getAudioDeviceSetup(self: &AudioDeviceManager) -> UniquePtr<AudioDeviceSetup>;

        #[rust_name = "set_audio_device_setup"]
        pub fn setAudioDeviceSetup(self: Pin<&mut AudioDeviceManager>, setup: &AudioDeviceSetup);

        #[rust_name = "get_current_audio_device"]
        pub fn getCurrentAudioDevice(self: Pin<&mut AudioDeviceManager>) -> *mut AudioIODevice;

        #[rust_name = "get_available_device_types"]
        pub fn getAvailableDeviceTypes(
            self: Pin<&mut AudioDeviceManager>,
        ) -> &AudioIODeviceTypeArray;

        #[rust_name = "get_current_device_type_object"]
        pub fn getCurrentDeviceTypeObject(self: &AudioDeviceManager) -> *mut AudioIODeviceType;

        #[rust_name = "play_test_sound"]
        pub fn playTestSound(self: Pin<&mut AudioDeviceManager>);

        #[rust_name = "add_audio_callback"]
        pub fn addAudioCallback(
            self: Pin<&mut AudioDeviceManager>,
            callback: &UniquePtr<AudioCallbackWrapper>,
        );

        #[rust_name = "remove_audio_callback"]
        pub fn removeAudioCallback(
            self: Pin<&mut AudioDeviceManager>,
            callback: &UniquePtr<AudioCallbackWrapper>,
        );

        #[rust_name = "add_audio_device_type"]
        pub fn addAudioDeviceType(
            self: Pin<&mut AudioDeviceManager>,
            device_type: Box<BoxedAudioIODeviceType>,
        );

        #[rust_name = "set_current_audio_device_type"]
        pub fn setCurrentAudioDeviceType(self: Pin<&mut AudioDeviceManager>, device_type: &str);

        #[namespace = "juce"]
        pub type AudioIODevice;

        #[namespace = "cxx_juce::audio_io_device"]
        #[rust_name = "get_device_name"]
        pub fn getDeviceName(self_: &AudioIODevice) -> &str;

        #[namespace = "cxx_juce::audio_io_device"]
        #[rust_name = "get_device_type_name"]
        pub fn getDeviceTypeName(self_: &AudioIODevice) -> &str;

        #[rust_name = "get_current_sample_rate"]
        pub fn getCurrentSampleRate(self: Pin<&mut AudioIODevice>) -> f64;

        #[rust_name = "get_current_buffer_size_samples"]
        pub fn getCurrentBufferSizeSamples(self: Pin<&mut AudioIODevice>) -> i32;

        #[namespace = "cxx_juce::audio_io_device"]
        #[rust_name = "get_available_sample_rates"]
        pub fn getAvailableSampleRates(self_: Pin<&mut AudioIODevice>) -> Vec<f64>;

        #[namespace = "cxx_juce::audio_io_device"]
        #[rust_name = "get_available_buffer_sizes"]
        pub fn getAvailableBufferSizes(self_: Pin<&mut AudioIODevice>) -> Vec<usize>;

        #[namespace = "cxx_juce::audio_io_device"]
        #[rust_name = "open"]
        pub fn open(
            self_: Pin<&mut AudioIODevice>,
            sample_rate: f64,
            buffer_size: usize,
        ) -> Result<()>;

        #[rust_name = "close"]
        pub fn close(self: Pin<&mut AudioIODevice>);

        #[namespace = "cxx_juce::audio_io_device"]
        #[rust_name = "count_active_input_channels"]
        pub fn countActiveInputChannels(self_: &AudioIODevice) -> i32;

        #[namespace = "cxx_juce::audio_io_device"]
        #[rust_name = "count_active_output_channels"]
        pub fn countActiveOutputChannels(self_: &AudioIODevice) -> i32;

        #[namespace = "juce"]
        pub type AudioIODeviceType;

        #[namespace = "cxx_juce::audio_io_device_type"]
        #[rust_name = "get_type_name"]
        pub fn getTypeName(self_: &AudioIODeviceType) -> String;

        #[rust_name = "scan_for_devices"]
        pub fn scanForDevices(self: Pin<&mut AudioIODeviceType>);

        #[namespace = "cxx_juce::audio_io_device_type"]
        #[rust_name = "get_input_device_names"]
        pub fn getInputDeviceNames(self_: &AudioIODeviceType) -> Vec<String>;

        #[namespace = "cxx_juce::audio_io_device_type"]
        #[rust_name = "get_output_device_names"]
        pub fn getOutputDeviceNames(self_: &AudioIODeviceType) -> Vec<String>;

        #[namespace = "cxx_juce::audio_io_device_type"]
        #[rust_name = "new_device"]
        pub fn createDevice(
            self_: Pin<&mut AudioIODeviceType>,
            input_device_name: &str,
            output_device_name: &str,
        ) -> UniquePtr<AudioIODevice>;

        #[namespace = "juce"]
        pub type AudioSampleBuffer;

        #[rust_name = "get_num_channels"]
        pub fn getNumChannels(self: &AudioSampleBuffer) -> i32;

        #[rust_name = "get_num_samples"]
        pub fn getNumSamples(self: &AudioSampleBuffer) -> i32;

        #[rust_name = "get_read_pointer"]
        pub fn getReadPointer(self: &AudioSampleBuffer, channel: i32) -> *const f32;

        #[rust_name = "get_write_pointer"]
        pub fn getWritePointer(self: Pin<&mut AudioSampleBuffer>, channel: i32) -> *mut f32;

        #[rust_name = "clear"]
        pub fn clear(self: Pin<&mut AudioSampleBuffer>);

        pub type AudioCallbackWrapper;

        #[namespace = "cxx_juce::system_audio_volume"]
        #[rust_name = "set_muted"]
        pub fn setMuted(muted: bool);

        #[namespace = "cxx_juce::system_audio_volume"]
        #[rust_name = "is_muted"]
        pub fn isMuted() -> bool;

        #[namespace = "cxx_juce::system_audio_volume"]
        #[rust_name = "set_gain"]
        pub fn setGain(gain: f32);

        #[namespace = "cxx_juce::system_audio_volume"]
        #[rust_name = "get_gain"]
        pub fn getGain() -> f32;

        #[namespace = "juce"]
        pub type SingleThreadedIIRFilter;

        #[namespace = "cxx_juce::iir_filter"]
        #[rust_name = "create_iir_filter"]
        pub fn createIIRFilter(coefficients: [f32; 5]) -> UniquePtr<SingleThreadedIIRFilter>;

        #[namespace = "juce"]
        #[rust_name = "process_samples"]
        pub unsafe fn processSamples(
            self: Pin<&mut SingleThreadedIIRFilter>,
            samples: *mut f32,
            num_samples: i32,
        );

        #[namespace = "cxx_juce::iir_filter"]
        #[rust_name = "make_low_pass"]
        pub fn makeLowPass(sample_rate: f64, frequency: f64, q: f64) -> [f32; 5];

        #[namespace = "cxx_juce::iir_filter"]
        #[rust_name = "make_high_pass"]
        pub fn makeHighPass(sample_rate: f64, frequency: f64, q: f64) -> [f32; 5];

        #[namespace = "cxx_juce::iir_filter"]
        #[rust_name = "make_notch_filter"]
        pub fn makeNotchFilter(sample_rate: f64, frequency: f64, q: f64) -> [f32; 5];
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
