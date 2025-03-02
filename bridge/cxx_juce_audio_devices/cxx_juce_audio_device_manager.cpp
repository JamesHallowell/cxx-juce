#include <cxx_juce_bindings.h>

namespace cxx_juce
{
void AudioDeviceManager::initialiseWithDefaultDevices (rust::i32 inputChannels,
                                                       rust::i32 outputChannels)
{
    const auto result = _audioDeviceManager.initialiseWithDefaultDevices (inputChannels, outputChannels);
    if (result.isNotEmpty())
    {
        throw std::runtime_error (result.toStdString());
    }
}

[[nodiscard]] std::unique_ptr<AudioDeviceSetup> AudioDeviceManager::getAudioDeviceSetup() const
{
    return std::make_unique<AudioDeviceSetup> (_audioDeviceManager.getAudioDeviceSetup());
}

void AudioDeviceManager::setAudioDeviceSetup (const AudioDeviceSetup& setup)
{
    _audioDeviceManager.setAudioDeviceSetup (setup._audioDeviceSetup, true);
}

void AudioDeviceManager::addAudioCallback (const std::unique_ptr<AudioCallbackWrapper>& callback)
{
    _audioDeviceManager.addAudioCallback (callback.get());
}

void AudioDeviceManager::removeAudioCallback (const std::unique_ptr<AudioCallbackWrapper>& callback)
{
    _audioDeviceManager.removeAudioCallback (callback.get());
}

void AudioDeviceManager::addAudioDeviceType (rust::Box<BoxedAudioIODeviceType> audioIODeviceType)
{
    struct RustAudioIODeviceType : juce::AudioIODeviceType
    {
        explicit RustAudioIODeviceType (rust::Box<BoxedAudioIODeviceType> audioIODeviceType)
            : juce::AudioIODeviceType (
                  static_cast<std::string> (::audio_io_device_type::name (*audioIODeviceType)))
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

void AudioDeviceManager::setCurrentAudioDeviceType (rust::Str audioDeviceTypeName)
{
    _audioDeviceManager.setCurrentAudioDeviceType (static_cast<std::string> (audioDeviceTypeName), true);
}

void AudioDeviceManager::playTestSound()
{
    _audioDeviceManager.playTestSound();
}

juce::AudioIODevice* AudioDeviceManager::getCurrentAudioDevice()
{
    return _audioDeviceManager.getCurrentAudioDevice();
}

const juce::OwnedArray<juce::AudioIODeviceType>& AudioDeviceManager::getAvailableDeviceTypes()
{
    return _audioDeviceManager.getAvailableDeviceTypes();
}

juce::AudioIODeviceType* AudioDeviceManager::getCurrentDeviceTypeObject() const
{
    return _audioDeviceManager.getCurrentDeviceTypeObject();
}

std::unique_ptr<AudioDeviceManager> createAudioDeviceManager()
{
    jassert (juce::MessageManager::getInstanceWithoutCreating());
    return std::make_unique<AudioDeviceManager>();
}

} // namespace cxx_juce
