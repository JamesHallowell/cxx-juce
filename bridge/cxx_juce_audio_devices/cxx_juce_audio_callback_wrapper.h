#pragma once

#include <juce_audio_devices/juce_audio_devices.h>
#include <rust/cxx.h>

namespace cxx_juce
{

struct BoxedAudioIODeviceCallback;

class AudioCallbackWrapper : public juce::AudioIODeviceCallback
{
public:
    explicit AudioCallbackWrapper (rust::Box<BoxedAudioIODeviceCallback> callback);

    void audioDeviceAboutToStart (juce::AudioIODevice* device) override;
    void audioDeviceIOCallbackWithContext (const float* const* inputChannelData,
                                           int numInputChannels,
                                           float* const* outputChannelData,
                                           int numOutputChannels,
                                           int numSamples,
                                           const juce::AudioIODeviceCallbackContext& context) override;
    void audioDeviceStopped() override;

private:
    rust::Box<BoxedAudioIODeviceCallback> _callback;
};

std::unique_ptr<AudioCallbackWrapper> wrapAudioCallback (rust::Box<BoxedAudioIODeviceCallback> callback);

} // namespace cxx_juce
