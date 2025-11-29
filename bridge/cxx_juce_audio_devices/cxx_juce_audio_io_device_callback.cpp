#include "cxx_juce_audio_io_device_callback.h"

#include <cxx-juce/src/juce_audio_devices/device_callback.rs.h>
#include <cxx-juce/src/juce_audio_devices/mod.rs.h>

namespace cxx_juce
{
void DropBoxDynAudioDeviceCallback::operator() (BoxDynAudioDeviceCallback* callback) const
{
    BoxDynAudioDeviceCallbackImpl::drop (callback);
}

std::unique_ptr<juce::AudioIODeviceCallback> wrapAudioDeviceCallback (BoxDynAudioDeviceCallback callback)
{
    struct AudioIODeviceCallback : juce::AudioIODeviceCallback
    {
        explicit AudioIODeviceCallback (BoxDynAudioDeviceCallback callback)
            : _callback { std::move (callback) }
        {
        }

        void audioDeviceAboutToStart (juce::AudioIODevice* device) override
        {
            BoxDynAudioDeviceCallbackImpl::about_to_start (_callback, *device);
        }

        void audioDeviceIOCallbackWithContext (const float* const* inputChannelData,
                                               int numInputChannels,
                                               float* const* outputChannelData,
                                               int numOutputChannels,
                                               int numSamples,
                                               const juce::AudioIODeviceCallbackContext&) override
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

            BoxDynAudioDeviceCallbackImpl::process_block (_callback, inputBuffer, outputBuffer);
        }

        void audioDeviceStopped() override
        {
            BoxDynAudioDeviceCallbackImpl::stopped (_callback);
        }

        BoxDynAudioDeviceCallback _callback;
    };

    return std::make_unique<AudioIODeviceCallback> (std::move (callback));
}

} // namespace cxx_juce
