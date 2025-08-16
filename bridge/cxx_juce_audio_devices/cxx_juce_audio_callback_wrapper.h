#pragma once

#include <juce_audio_devices/juce_audio_devices.h>
#include <rust/cxx.h>

namespace cxx_juce
{

class BoxDynAudioIODeviceCallback {
public:
    BoxDynAudioIODeviceCallback(BoxDynAudioIODeviceCallback &&) noexcept;
    ~BoxDynAudioIODeviceCallback() noexcept;
    using IsRelocatable = std::true_type;

private:
    using FatPtr = std::array<uintptr_t, 2>;
    FatPtr repr;
};

class AudioCallbackWrapper : public juce::AudioIODeviceCallback
{
public:
    explicit AudioCallbackWrapper (BoxDynAudioIODeviceCallback callback);

    void audioDeviceAboutToStart (juce::AudioIODevice* device) override;
    void audioDeviceIOCallbackWithContext (const float* const* inputChannelData,
                                           int numInputChannels,
                                           float* const* outputChannelData,
                                           int numOutputChannels,
                                           int numSamples,
                                           const juce::AudioIODeviceCallbackContext& context) override;
    void audioDeviceStopped() override;

private:
    BoxDynAudioIODeviceCallback _callback;
};

std::unique_ptr<AudioCallbackWrapper> wrapAudioCallback (BoxDynAudioIODeviceCallback callback);

} // namespace cxx_juce
