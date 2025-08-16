#include <cxx_juce_bindings.h>

namespace cxx_juce
{

BoxDynAudioIODeviceCallback::BoxDynAudioIODeviceCallback (BoxDynAudioIODeviceCallback&& other) noexcept :
    repr {other.repr}
{
    other.repr = {0, 0};
}

BoxDynAudioIODeviceCallback::~BoxDynAudioIODeviceCallback() noexcept {
    if (repr != FatPtr {0, 0})
    {
        audio_io_device_callback::drop (this);
    }
}

AudioCallbackWrapper::AudioCallbackWrapper (BoxDynAudioIODeviceCallback callback)
    : _callback (std::move (callback))
{
}

void AudioCallbackWrapper::audioDeviceIOCallbackWithContext (
    const float* const* inputChannelData,
    int numInputChannels,
    float* const* outputChannelData,
    int numOutputChannels,
    int numSamples,
    const juce::AudioIODeviceCallbackContext&)
{
    juce::AudioSampleBuffer inputBuffer;
    if (inputChannelData)
    {
        inputBuffer.setDataToReferTo (const_cast<float* const*> (inputChannelData),
                                      numInputChannels,
                                      numSamples);
    }

    juce::AudioSampleBuffer outputBuffer;
    if (outputChannelData)
    {
        outputBuffer.setDataToReferTo (outputChannelData,
                                       numOutputChannels,
                                       numSamples);
    }

    audio_io_device_callback::processBlock (_callback,
                                            inputBuffer,
                                            outputBuffer);
}

void AudioCallbackWrapper::audioDeviceAboutToStart (juce::AudioIODevice* device)
{
    if (! device)
    {
        return;
    }

    audio_io_device_callback::aboutToStart (_callback,
                                            *device);
}

void AudioCallbackWrapper::audioDeviceStopped()
{
    audio_io_device_callback::stopped (_callback);
}

std::unique_ptr<AudioCallbackWrapper> wrapAudioCallback (BoxDynAudioIODeviceCallback callback)
{
    return std::make_unique<AudioCallbackWrapper> (std::move (callback));
}

} // namespace cxx_juce
