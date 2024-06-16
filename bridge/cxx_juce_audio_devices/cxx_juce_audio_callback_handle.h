#pragma once

#include "juce_audio_devices/juce_audio_devices.h"
#include <rust/cxx.h>

namespace cxx_juce
{

struct BoxedAudioIODeviceCallback;

class AudioCallbackHandle : public juce::AudioIODeviceCallback
{
public:
    explicit AudioCallbackHandle (juce::AudioDeviceManager& audioDeviceManager,
                                  rust::Box<BoxedAudioIODeviceCallback> callback);
    ~AudioCallbackHandle() override;

    void audioDeviceAboutToStart (juce::AudioIODevice* device) override;
    void audioDeviceIOCallbackWithContext (const float* const* inputChannelData,
                                           int numInputChannels,
                                           float* const* outputChannelData,
                                           int numOutputChannels,
                                           int numSamples,
                                           const juce::AudioIODeviceCallbackContext& context) override;
    void audioDeviceStopped() override;

private:
    juce::AudioDeviceManager& _audioDeviceManager;
    rust::Box<BoxedAudioIODeviceCallback> _callback;
};

} // namespace cxx_juce
