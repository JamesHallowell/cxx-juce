#include "cxx_juce_audio_io_device.h"

#include <cxx-juce/src/juce_audio_devices/device.rs.h>
#include <cxx-juce/src/juce_audio_devices/mod.rs.h>

namespace cxx_juce
{

BoxDynAudioDevice::BoxDynAudioDevice (BoxDynAudioDevice&& other) noexcept
    : _repr { other._repr }
{
    other._repr = { 0, 0 };
}

BoxDynAudioDevice::~BoxDynAudioDevice() noexcept
{
    if (_repr != FatPtr { 0, 0 })
    {
        BoxDynAudioIODeviceImpl::drop (this);
    }
}

std::unique_ptr<juce::AudioIODevice> wrapAudioDevice (BoxDynAudioDevice device)
{
    struct AudioIODevice : juce::AudioIODevice
    {
        explicit AudioIODevice (BoxDynAudioDevice device)
            : juce::AudioIODevice (
                  static_cast<std::string> (BoxDynAudioIODeviceImpl::name (device)),
                  static_cast<std::string> (BoxDynAudioIODeviceImpl::type_name (device)))
            , _device { std::move (device) }
        {
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
            for (auto sampleRate : BoxDynAudioIODeviceImpl::available_sample_rates (_device))
            {
                sampleRates.add (sampleRate);
            }
            return sampleRates;
        }

        juce::Array<int> getAvailableBufferSizes() override
        {
            juce::Array<int> bufferSizes;
            for (auto bufferSize : BoxDynAudioIODeviceImpl::available_buffer_sizes (_device))
            {
                bufferSizes.add (bufferSize);
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
            return BoxDynAudioIODeviceImpl::open (_device, sampleRate, bufferSize);
        }

        void close() override
        {
            BoxDynAudioIODeviceImpl::close (_device);
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
            return BoxDynAudioIODeviceImpl::buffer_size (_device);
        }

        double getCurrentSampleRate() override
        {
            return BoxDynAudioIODeviceImpl::sample_rate (_device);
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

        BoxDynAudioDevice _device;
    };

    return std::make_unique<AudioIODevice> (std::move (device));
}
} // namespace cxx_juce
