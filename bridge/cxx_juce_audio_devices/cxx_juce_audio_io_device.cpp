#include "cxx_juce_audio_io_device.h"

#include <cxx-juce/src/juce_audio_devices/device.rs.h>
#include <cxx-juce/src/juce_audio_devices/mod.rs.h>

namespace cxx_juce
{

CXX_JUCE_DEFINE_BOXED_TRAIT_TYPE (AudioDevice)

std::unique_ptr<juce::AudioIODevice> wrap (BoxDynAudioDevice device) noexcept
{
    struct AudioIODevice : juce::AudioIODevice
    {
        explicit AudioIODevice (BoxDynAudioDevice device)
            : juce::AudioIODevice (
                  static_cast<std::string> (AudioDeviceImpl::name (device)),
                  static_cast<std::string> (AudioDeviceImpl::type_name (device)))
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
            for (auto sampleRate : AudioDeviceImpl::available_sample_rates (_device))
            {
                sampleRates.add (sampleRate);
            }
            return sampleRates;
        }

        juce::Array<int> getAvailableBufferSizes() override
        {
            juce::Array<int> bufferSizes;
            for (auto bufferSize : AudioDeviceImpl::available_buffer_sizes (_device))
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
            return AudioDeviceImpl::open (_device, sampleRate, bufferSize);
        }

        void close() override
        {
            AudioDeviceImpl::close (_device);
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
            return AudioDeviceImpl::buffer_size (_device);
        }

        double getCurrentSampleRate() override
        {
            return AudioDeviceImpl::sample_rate (_device);
        }

        int getCurrentBitDepth() override
        {
            return 0;
        }

        [[nodiscard]] juce::BigInteger getActiveOutputChannels() const override
        {
            return AudioDeviceImpl::output_channels (_device);
        }

        [[nodiscard]] juce::BigInteger getActiveInputChannels() const override
        {
            return AudioDeviceImpl::input_channels (_device);
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
