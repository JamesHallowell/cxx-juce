use std::pin::Pin;

pub use juce::AudioSampleBuffer;

impl AudioSampleBuffer {
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
        type AudioSampleBuffer;

        #[rust_name = "get_num_channels"]
        fn getNumChannels(self: &AudioSampleBuffer) -> i32;

        #[rust_name = "get_num_samples"]
        fn getNumSamples(self: &AudioSampleBuffer) -> i32;

        #[rust_name = "get_read_pointer"]
        fn getReadPointer(self: &AudioSampleBuffer, channel: i32) -> *const f32;

        #[rust_name = "get_write_pointer"]
        fn getWritePointer(self: Pin<&mut AudioSampleBuffer>, channel: i32) -> *mut f32;

        #[rust_name = "clear"]
        fn clear(self: Pin<&mut AudioSampleBuffer>);
    }
}
