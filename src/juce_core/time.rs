use crate::define_juce_type;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

define_juce_type! {
    #[derive(Copy, Clone)]
    /// A point in time.
    Time,
    layout = juce::TimeLayout,
    cxx_name = "juce::Time",
    send,
    debug = Time::to_milliseconds,
}

impl Time {
    /// Returns the time as milliseconds since the Unix epoch.
    pub fn to_milliseconds(&self) -> i64 {
        juce::to_milliseconds(self)
    }
}

impl From<SystemTime> for Time {
    fn from(time: SystemTime) -> Self {
        juce::time_new(match time.duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_millis() as i64,
            Err(err) => -(err.duration().as_millis() as i64),
        })
    }
}

impl From<Time> for SystemTime {
    fn from(time: Time) -> Self {
        match time.to_milliseconds() {
            millis if millis > 0 => {
                UNIX_EPOCH + Duration::from_millis(time.to_milliseconds() as u64)
            }
            millis if millis < 0 => {
                UNIX_EPOCH - Duration::from_millis((-time.to_milliseconds()) as u64)
            }
            _ => UNIX_EPOCH,
        }
    }
}

impl PartialEq<Time> for Time {
    fn eq(&self, other: &Time) -> bool {
        self.to_milliseconds() == other.to_milliseconds()
    }
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum TimeLayout {
        Size = 8,
        Alignment = 8,
    }

    unsafe extern "C++" {
        include!("cxx_juce.h");

        type Time = super::Time;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn time_new(milliseconds_since_epoch: i64) -> Time;

        #[cxx_name = "toMilliseconds"]
        fn to_milliseconds(self_: &Time) -> i64;
    }
}
