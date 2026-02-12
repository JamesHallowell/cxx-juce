use std::pin::Pin;

pub use juce::AudioSampleBuffer;

impl AudioSampleBuffer {
    /// Returns the samples for the given channel as a read-only slice.
    pub fn get_read_slice(&self, channel: i32) -> &[f32] {
        if channel >= self.get_num_channels() {
            return &[];
        }

        let ptr = self.get_read_pointer(channel);

        self.get_num_samples()
            .try_into()
            .map(|samples| unsafe { std::slice::from_raw_parts(ptr, samples) })
            .unwrap_or(&[])
    }

    /// Returns the samples for the given channel as a mutable slice.
    pub fn get_write_slice(mut self: Pin<&mut Self>, channel: i32) -> &mut [f32] {
        if channel >= self.as_ref().get_num_channels() {
            return &mut [];
        }

        let ptr = self.as_mut().get_write_pointer(channel);

        self.get_num_samples()
            .try_into()
            .map(|samples| unsafe { std::slice::from_raw_parts_mut(ptr, samples) })
            .unwrap_or(&mut [])
    }
}

#[cxx::bridge(namespace = "cxx_juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        #[namespace = "juce"]
        /// A multi-channel buffer of floating point audio samples.
        type AudioSampleBuffer;

        /// Returns the number of channels in the buffer.
        #[rust_name = "get_num_channels"]
        fn getNumChannels(self: &AudioSampleBuffer) -> i32;

        /// Returns the number of samples in each channel.
        #[rust_name = "get_num_samples"]
        fn getNumSamples(self: &AudioSampleBuffer) -> i32;

        /// Returns a read-only pointer to the samples for the given channel.
        #[rust_name = "get_read_pointer"]
        fn getReadPointer(self: &AudioSampleBuffer, channel: i32) -> *const f32;

        /// Returns a writable pointer to the samples for the given channel.
        #[rust_name = "get_write_pointer"]
        fn getWritePointer(self: Pin<&mut AudioSampleBuffer>, channel: i32) -> *mut f32;

        /// Clears all the samples in all channels.
        #[rust_name = "clear"]
        fn clear(self: Pin<&mut AudioSampleBuffer>);
    }
}
