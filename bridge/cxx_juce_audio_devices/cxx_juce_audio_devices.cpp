#include <cxx_juce_audio_devices/cxx_juce_audio_devices.h>

#include <cxx-juce/src/juce_audio_devices/device.rs.h>
#include <cxx-juce/src/juce_audio_devices/device_manager.rs.h>
#include <cxx-juce/src/juce_audio_devices/device_callback.rs.h>
#include <cxx-juce/src/juce_audio_devices/device_type.rs.h>

#include <cxx_juce_utils.h>

CXX_JUCE_ASSERT_SIZE_ALIGN (AudioDeviceSetup)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceSetup, outputDeviceName, OutputDeviceNameOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceSetup, inputDeviceName, InputDeviceNameOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceSetup, sampleRate, SampleRateOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceSetup, bufferSize, BufferSizeOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceSetup, inputChannels, InputChannelsOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceSetup, useDefaultInputChannels, UseDefaultInputChannelsOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceSetup, outputChannels, OutputChannelsOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceSetup, useDefaultOutputChannels, UseDefaultOutputChannelsOffset)

namespace cxx_juce
{
CXX_JUCE_DEFINE_BOXED_TRAIT_TYPE (AudioDevice)
CXX_JUCE_DEFINE_BOXED_TRAIT_TYPE (AudioDeviceCallback)
CXX_JUCE_DEFINE_BOXED_TRAIT_TYPE (AudioDeviceType)

std::unique_ptr<juce::AudioIODevice> wrap (BoxDynAudioDevice device) noexcept
{
    struct AudioDevice : juce::AudioIODevice
    {
        explicit AudioDevice (BoxDynAudioDevice device)
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

    return std::make_unique<AudioDevice> (std::move (device));
}

std::unique_ptr<juce::AudioIODeviceCallback> wrap (BoxDynAudioDeviceCallback callback) noexcept
{
    struct AudioDeviceCallback : juce::AudioIODeviceCallback
    {
        explicit AudioDeviceCallback (BoxDynAudioDeviceCallback callback)
            : _callback { std::move (callback) }
        {
        }

        void audioDeviceAboutToStart (juce::AudioIODevice* device) override
        {
            AudioDeviceCallbackImpl::about_to_start (_callback, *device);
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

            AudioDeviceCallbackImpl::process_block (_callback, inputBuffer, outputBuffer);
        }

        void audioDeviceStopped() override
        {
            AudioDeviceCallbackImpl::stopped (_callback);
        }

        BoxDynAudioDeviceCallback _callback;
    };

    return std::make_unique<AudioDeviceCallback> (std::move (callback));
}


std::unique_ptr<juce::AudioIODeviceType> wrap (BoxDynAudioDeviceType deviceType) noexcept
{
    struct AudioDeviceType : juce::AudioIODeviceType
    {
        explicit AudioDeviceType (BoxDynAudioDeviceType deviceType)
            : juce::AudioIODeviceType (
                  static_cast<std::string> (AudioDeviceTypeImpl::name (deviceType)))
            , _deviceType { std::move (deviceType) }
        {
        }

        void scanForDevices() override
        {
            AudioDeviceTypeImpl::scan_for_devices (_deviceType);
        }

        [[nodiscard]] juce::StringArray getDeviceNames (bool wantInputNames) const override
        {
            const auto names = wantInputNames ? AudioDeviceTypeImpl::input_devices (_deviceType) : AudioDeviceTypeImpl::output_devices (_deviceType);

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

        juce::AudioIODevice* createDevice (const juce::String& inputDeviceName,
                                           const juce::String& outputDeviceName) override
        {
            try
            {
                return AudioDeviceTypeImpl::create_device (_deviceType, inputDeviceName, outputDeviceName).release();
            }
            catch (const rust::Error&)
            {
                return nullptr;
            }
        }

        BoxDynAudioDeviceType _deviceType;
    };

    return std::make_unique<AudioDeviceType> (std::move (deviceType));
}
} // namespace cxx_juce
