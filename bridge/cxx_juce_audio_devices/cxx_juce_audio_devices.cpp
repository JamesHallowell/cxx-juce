#include <cxx_juce_audio_devices/cxx_juce_audio_devices.h>

#include <cxx-juce/src/juce_audio_devices/device.rs.h>
#include <cxx-juce/src/juce_audio_devices/device_callback.rs.h>
#include <cxx-juce/src/juce_audio_devices/device_manager.rs.h>
#include <cxx-juce/src/juce_audio_devices/device_type.rs.h>
#include <cxx-juce/src/juce_audio_devices/midi_device_info.rs.h>
#include <cxx-juce/src/juce_audio_devices/midi_input.rs.h>
#include <cxx-juce/src/juce_audio_devices/midi_output.rs.h>

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

CXX_JUCE_ASSERT_SIZE_ALIGN (MidiDeviceInfo)
CXX_JUCE_ASSERT_FIELD_OFFSET (MidiDeviceInfo, name, NameOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (MidiDeviceInfo, identifier, IdentifierOffset)

namespace cxx_juce
{
CXX_JUCE_DEFINE_BOXED_TRAIT_TYPE (AudioDevice)
CXX_JUCE_DEFINE_BOXED_TRAIT_TYPE (AudioDeviceCallback)
CXX_JUCE_DEFINE_BOXED_TRAIT_TYPE (AudioDeviceType)
CXX_JUCE_DEFINE_BOXED_TRAIT_TYPE (MidiInputCallback)

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
            return AudioDeviceImpl::output_channel_names (_device);
        }

        juce::StringArray getInputChannelNames() override
        {
            return AudioDeviceImpl::input_channel_names (_device);
        }

        juce::Array<double> getAvailableSampleRates() override
        {
            return AudioDeviceImpl::available_sample_rates (_device);
        }

        juce::Array<int> getAvailableBufferSizes() override
        {
            return AudioDeviceImpl::available_buffer_sizes (_device);
        }

        int getDefaultBufferSize() override
        {
            return AudioDeviceImpl::default_buffer_size (_device);
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
            return AudioDeviceImpl::is_open (_device);
        }

        void start (juce::AudioIODeviceCallback* /*callback*/) override
        {
            AudioDeviceImpl::start (_device);
        }

        void stop() override
        {
            AudioDeviceImpl::stop (_device);
        }

        bool isPlaying() override
        {
            return AudioDeviceImpl::is_playing (_device);
        }

        juce::String getLastError() override
        {
            return static_cast<std::string> (AudioDeviceImpl::last_error (_device));
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
            return AudioDeviceImpl::bit_depth (_device);
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
            return AudioDeviceImpl::output_latency (_device);
        }

        int getInputLatencyInSamples() override
        {
            return AudioDeviceImpl::input_latency (_device);
        }

        [[nodiscard]] bool hasControlPanel() const override
        {
            return AudioDeviceImpl::has_control_panel (_device);
        }

        bool showControlPanel() override
        {
            return AudioDeviceImpl::show_control_panel (_device);
        }

        bool setAudioPreprocessingEnabled (bool enabled) override
        {
            return AudioDeviceImpl::set_audio_preprocessing_enabled (_device, enabled);
        }

        [[nodiscard]] int getXRunCount() const noexcept override
        {
            return AudioDeviceImpl::xrun_count (_device);
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
            auto names = wantInputNames ? AudioDeviceTypeImpl::input_devices (_deviceType)
                                        : AudioDeviceTypeImpl::output_devices (_deviceType);

            return names;
        }

        [[nodiscard]] int getDefaultDeviceIndex (bool forInput) const override
        {
            return AudioDeviceTypeImpl::default_device_index (_deviceType, forInput);
        }

        int getIndexOfDevice (juce::AudioIODevice* device,
                              bool asInput) const override
        {
            return getDeviceNames (asInput).indexOf (device->getName());
        }

        [[nodiscard]] bool hasSeparateInputsAndOutputs() const override
        {
            return AudioDeviceTypeImpl::has_separate_inputs_and_outputs (_deviceType);
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

std::unique_ptr<juce::MidiInputCallback> wrap (BoxDynMidiInputCallback callback) noexcept
{
    struct MidiInputCallback : juce::MidiInputCallback
    {
        explicit MidiInputCallback (BoxDynMidiInputCallback callback)
            : _callback { std::move (callback) }
        {
        }

        void handleIncomingMidiMessage (juce::MidiInput*,
                                        const juce::MidiMessage& message) override
        {
            MidiInputCallbackImpl::handle_incoming_midi_message (
                _callback,
                message);
        }

        BoxDynMidiInputCallback _callback;
    };

    return std::make_unique<MidiInputCallback> (std::move (callback));
}
} // namespace cxx_juce
