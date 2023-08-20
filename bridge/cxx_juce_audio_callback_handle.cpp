#include "cxx_juce_bindings.h"

namespace cxx_juce
{
AudioCallbackHandle::AudioCallbackHandle (juce::AudioDeviceManager& audioDeviceManager, rust::Box<BoxedAudioIODeviceCallback> callback)
    : _audioDeviceManager (audioDeviceManager)
    , _callback (std::move (callback))
{
    _audioDeviceManager.addAudioCallback (this);
}

AudioCallbackHandle::~AudioCallbackHandle()
{
    _audioDeviceManager.removeAudioCallback (this);
}

void AudioCallbackHandle::audioDeviceIOCallbackWithContext (
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

    ::audio_io_device_callback::processBlock (*_callback,
                                              inputBuffer,
                                              outputBuffer);
}

void AudioCallbackHandle::audioDeviceAboutToStart (juce::AudioIODevice* device)
{
    if (! device)
    {
        return;
    }

    ::audio_io_device_callback::aboutToStart (*_callback,
                                              *device);
}

void AudioCallbackHandle::audioDeviceStopped()
{
    ::audio_io_device_callback::stopped (*_callback);
}
} // namespace cxx_juce