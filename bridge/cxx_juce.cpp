#if __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdollar-in-identifier-extension"
#pragma clang diagnostic ignored "-Wmissing-prototypes"
#endif

#include "cxx-juce/src/lib.rs.cc"
#include "cxx-juce/src/lib.rs.h"

#if __clang__
#pragma clang diagnostic pop
#endif

#include <juce_audio_devices/juce_audio_devices.h>
#include <juce_core/juce_core.h>

namespace cxx_juce
{
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

void AudioDeviceManager::addAudioDeviceType (rust::Box<BoxedAudioIODeviceType> audioIODeviceType)
{
    struct RustAudioIODeviceType : juce::AudioIODeviceType
    {
        explicit RustAudioIODeviceType (rust::Box<BoxedAudioIODeviceType> audioIODeviceType)
            : juce::AudioIODeviceType (static_cast<std::string> (::audio_io_device_type::name (*audioIODeviceType)))
            , _audioIODeviceType (std::move (audioIODeviceType))
        {
        }

        void scanForDevices() override
        {
            ::audio_io_device_type::scanForDevices (*_audioIODeviceType);
        }

        [[nodiscard]] juce::StringArray getDeviceNames (bool wantInputNames) const override
        {
            const auto names = ::audio_io_device_type::getDeviceNames (*_audioIODeviceType, wantInputNames);

            juce::StringArray stringArray;
            for (const auto& name : names)
            {
                stringArray.add (static_cast<std::string> (name));
            }
            return stringArray;
        }

        [[nodiscard]] int getDefaultDeviceIndex (bool /*forInput*/) const override
        {
            return 0;
        }

        int getIndexOfDevice (juce::AudioIODevice* device,
                              bool asInput) const override
        {
            return getDeviceNames (asInput).indexOf (device->getName());
        }

        [[nodiscard]] bool hasSeparateInputsAndOutputs() const override
        {
            return true;
        }

        juce::AudioIODevice* createDevice (const juce::String& outputDeviceName,
                                           const juce::String& inputDeviceName) override
        {
            struct RustAudioIODevice : juce::AudioIODevice
            {
                explicit RustAudioIODevice (BoxedAudioIODevice* device)
                    : juce::AudioIODevice (
                        static_cast<std::string> (::audio_io_device::deviceName (*device)),
                        static_cast<std::string> (::audio_io_device::typeName (*device)))
                    , _device (device)
                {
                }

                ~RustAudioIODevice() override
                {
                    ::audio_io_device_type::destroyDevice (_device);
                }

                juce::StringArray getOutputChannelNames() override
                {
                    return {};
                }

                juce::StringArray getInputChannelNames() override
                {
                    return {};
                }

                juce::Array<double> getAvailableSampleRates() override
                {
                    juce::Array<double> sampleRates;
                    for (auto sampleRate : ::audio_io_device::availableSampleRates (*_device))
                    {
                        sampleRates.add (sampleRate);
                    }
                    return sampleRates;
                }

                juce::Array<int> getAvailableBufferSizes() override
                {
                    juce::Array<int> bufferSizes;
                    for (auto bufferSize : ::audio_io_device::availableBufferSizes (*_device))
                    {
                        bufferSizes.add (static_cast<int> (bufferSize));
                    }
                    return bufferSizes;
                }

                int getDefaultBufferSize() override
                {
                    return 0;
                }

                juce::String open (const juce::BigInteger& /*inputChannels*/,
                                   const juce::BigInteger& /*outputChannels*/,
                                   double sampleRate,
                                   int bufferSize) override
                {
                    const auto result = ::audio_io_device::open (*_device,
                                                                 sampleRate,
                                                                 static_cast<size_t> (bufferSize));
                    return static_cast<std::string> (result);
                }

                void close() override
                {
                    ::audio_io_device::close (*_device);
                }

                bool isOpen() override
                {
                    return false;
                }

                void start (juce::AudioIODeviceCallback* /*callback*/) override
                {
                }

                void stop() override
                {
                }

                bool isPlaying() override
                {
                    return false;
                }

                juce::String getLastError() override
                {
                    return {};
                }

                int getCurrentBufferSizeSamples() override
                {
                    return static_cast<int> (::audio_io_device::bufferSize (*_device));
                }

                double getCurrentSampleRate() override
                {
                    return ::audio_io_device::sampleRate (*_device);
                }

                int getCurrentBitDepth() override
                {
                    return 0;
                }

                [[nodiscard]] juce::BigInteger getActiveOutputChannels() const override
                {
                    return {};
                }

                [[nodiscard]] juce::BigInteger getActiveInputChannels() const override
                {
                    return {};
                }

                int getOutputLatencyInSamples() override
                {
                    return 0;
                }

                int getInputLatencyInSamples() override
                {
                    return 0;
                }

                [[nodiscard]] bool hasControlPanel() const override
                {
                    return false;
                }

                bool showControlPanel() override
                {
                    return false;
                }

                bool setAudioPreprocessingEnabled (bool) override
                {
                    return false;
                }

                [[nodiscard]] int getXRunCount() const noexcept override
                {
                    return 0;
                }

                BoxedAudioIODevice* _device { nullptr };
            };

            const auto device = ::audio_io_device_type::createDevice (
                *_audioIODeviceType,
                outputDeviceName.toStdString(),
                inputDeviceName.toStdString());

            if (! device)
            {
                return nullptr;
            }

            return std::make_unique<RustAudioIODevice> (device).release();
        }

        rust::Box<BoxedAudioIODeviceType> _audioIODeviceType;
    };

    _audioDeviceManager.addAudioDeviceType (std::make_unique<RustAudioIODeviceType> (std::move (audioIODeviceType)));
}
} // namespace cxx_juce