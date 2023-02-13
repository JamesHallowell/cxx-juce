#pragma once

#include "juce_audio_devices/juce_audio_devices.h"
#include "juce_core/juce_core.h"
#include "juce_events/juce_events.h"

#include "rust/cxx.h"

namespace juce
{
using AudioIODeviceTypeArray = OwnedArray<juce::AudioIODeviceType>;
void initialiseNSApplication();
} // namespace juce

namespace cxx_juce
{
struct BoxedAudioIODeviceCallback;
struct BoxedAudioIODeviceType;

rust::String juceVersion()
{
    return juce::SystemStats::getJUCEVersion().toStdString();
}

void initialiseJuce()
{
    juce::initialiseJuce_GUI();
}

void shutdownJuce()
{
    juce::shutdownJuce_GUI();
}

rust::Str toStr (const juce::String& string)
{
    return { string.toRawUTF8(), string.getNumBytesAsUTF8() };
}

class AudioCallbackHandle : public juce::AudioIODeviceCallback
{
public:
    explicit AudioCallbackHandle (juce::AudioDeviceManager& audioDeviceManager,
                                  rust::Box<BoxedAudioIODeviceCallback> callback)
        : _audioDeviceManager (audioDeviceManager)
        , _callback (std::move (callback))
    {
        _audioDeviceManager.addAudioCallback (this);
    }

    ~AudioCallbackHandle() override
    {
        _audioDeviceManager.removeAudioCallback (this);
    }

    void audioDeviceAboutToStart (juce::AudioIODevice* device) override;
    void audioDeviceIOCallbackWithContext (const float* const* inputChannelData,
                                           int numInputChannels,
                                           float* const* outputChannelData,
                                           int numOutputChannels,
                                           int numSamples,
                                           const juce::AudioIODeviceCallbackContext& context) override;
    void audioDeviceStopped() override;

private:
    juce::AudioDeviceManager& _audioDeviceManager;
    rust::Box<BoxedAudioIODeviceCallback> _callback;
};

struct AudioDeviceSetup
{
    AudioDeviceSetup() = default;

    explicit AudioDeviceSetup (juce::AudioDeviceManager::AudioDeviceSetup audioDeviceSetup)
        : _audioDeviceSetup (std::move (audioDeviceSetup))
    {
    }

    [[nodiscard]] rust::Str outputDeviceName() const
    {
        return toStr (_audioDeviceSetup.outputDeviceName);
    }

    void setOutputDeviceName (rust::Str outputDeviceName)
    {
        _audioDeviceSetup.outputDeviceName = static_cast<std::string> (outputDeviceName);
    }

    [[nodiscard]] rust::Str inputDeviceName() const
    {
        return toStr (_audioDeviceSetup.inputDeviceName);
    }

    void setInputDeviceName (rust::Str inputDeviceName)
    {
        _audioDeviceSetup.inputDeviceName = static_cast<std::string> (inputDeviceName);
    }

    [[nodiscard]] rust::f64 sampleRate() const
    {
        return _audioDeviceSetup.sampleRate;
    }

    void setSampleRate (rust::f64 sampleRate)
    {
        _audioDeviceSetup.sampleRate = sampleRate;
    }

    [[nodiscard]] rust::i32 bufferSize() const
    {
        return _audioDeviceSetup.bufferSize;
    }

    void setBufferSize (rust::i32 bufferSize)
    {
        _audioDeviceSetup.bufferSize = bufferSize;
    }

    [[nodiscard]] rust::i32 numberOfInputChannels() const
    {
        return _audioDeviceSetup.inputChannels.countNumberOfSetBits();
    }

    void setNumberOfInputChannels (rust::i32 numberOfInputChannels)
    {
        _audioDeviceSetup.inputChannels.clear();
        _audioDeviceSetup.inputChannels.setRange (0, numberOfInputChannels, true);
    }

    void useDefaultInputChannels (bool useDefaultInputChannels)
    {
        _audioDeviceSetup.useDefaultInputChannels = useDefaultInputChannels;
    }

    [[nodiscard]] bool usingDefaultInputChannels() const
    {
        return _audioDeviceSetup.useDefaultInputChannels;
    }

    [[nodiscard]] rust::i32 numberOfOutputChannels() const
    {
        return _audioDeviceSetup.outputChannels.countNumberOfSetBits();
    }

    void setNumberOfOutputChannels (rust::i32 numberOfOutputChannels)
    {
        _audioDeviceSetup.outputChannels.clear();
        _audioDeviceSetup.outputChannels.setRange (0, numberOfOutputChannels, true);
    }

    void useDefaultOutputChannels (bool useDefaultOutputChannels)
    {
        _audioDeviceSetup.useDefaultOutputChannels = useDefaultOutputChannels;
    }

    [[nodiscard]] bool usingDefaultOutputChannels() const
    {
        return _audioDeviceSetup.useDefaultOutputChannels;
    }

    juce::AudioDeviceManager::AudioDeviceSetup _audioDeviceSetup;
};

std::unique_ptr<AudioDeviceSetup> createAudioDeviceSetup()
{
    return std::make_unique<AudioDeviceSetup>();
}

struct AudioDeviceManager
{
    void initialiseWithDefaultDevices (rust::i32 inputChannels,
                                       rust::i32 outputChannels)
    {
        const auto result = _audioDeviceManager.initialiseWithDefaultDevices (inputChannels, outputChannels);
        if (result.isNotEmpty())
        {
            throw std::runtime_error (result.toStdString());
        }
    }

    [[nodiscard]] std::unique_ptr<AudioDeviceSetup> getAudioDeviceSetup() const
    {
        return std::make_unique<AudioDeviceSetup> (_audioDeviceManager.getAudioDeviceSetup());
    }

    void setAudioDeviceSetup (const AudioDeviceSetup& setup)
    {
        _audioDeviceManager.setAudioDeviceSetup (setup._audioDeviceSetup, true);
    }

    [[nodiscard]] std::unique_ptr<AudioCallbackHandle>
        addAudioCallback (rust::Box<BoxedAudioIODeviceCallback> callback)
    {
        return std::make_unique<AudioCallbackHandle> (_audioDeviceManager, std::move (callback));
    }

    void addAudioDeviceType (rust::Box<BoxedAudioIODeviceType> audioIODeviceType);

    void setCurrentAudioDeviceType (rust::Str audioDeviceTypeName)
    {
        _audioDeviceManager.setCurrentAudioDeviceType (static_cast<std::string> (audioDeviceTypeName), true);
    }

    void playTestSound()
    {
        _audioDeviceManager.playTestSound();
    }

    juce::AudioIODevice* getCurrentAudioDevice() const
    {
        return _audioDeviceManager.getCurrentAudioDevice();
    }

    const juce::OwnedArray<juce::AudioIODeviceType>& getAvailableDeviceTypes()
    {
        return _audioDeviceManager.getAvailableDeviceTypes();
    }

    juce::AudioIODeviceType* getCurrentDeviceTypeObject() const
    {
        return _audioDeviceManager.getCurrentDeviceTypeObject();
    }

    juce::AudioDeviceManager _audioDeviceManager;
};

std::unique_ptr<AudioDeviceManager> createAudioDeviceManager()
{
    jassert (juce::MessageManager::getInstanceWithoutCreating());
    return std::make_unique<AudioDeviceManager>();
}

namespace audio_io_device_type
{
    rust::String getTypeName (const juce::AudioIODeviceType& audioIoDeviceType)
    {
        return audioIoDeviceType.getTypeName().toStdString();
    }

    rust::Vec<rust::String> getInputDeviceNames (
        const juce::AudioIODeviceType& audioIoDeviceType)
    {
        const auto deviceNames = audioIoDeviceType.getDeviceNames (true);

        rust::Vec<rust::String> result;
        std::transform (
            std::begin (deviceNames),
            std::end (deviceNames),
            std::back_inserter (result),
            [] (const auto& deviceName)
            { return deviceName.toStdString(); });
        return result;
    }

    rust::Vec<rust::String> getOutputDeviceNames (
        const juce::AudioIODeviceType& audioIoDeviceType)
    {
        const auto deviceNames = audioIoDeviceType.getDeviceNames (false);

        rust::Vec<rust::String> result;
        result.reserve (static_cast<size_t> (deviceNames.size()));
        std::transform (
            std::begin (deviceNames),
            std::end (deviceNames),
            std::back_inserter (result),
            [] (const auto& deviceName)
            { return deviceName.toStdString(); });
        return result;
    }

    std::unique_ptr<juce::AudioIODevice> createDevice (
        juce::AudioIODeviceType& audioIoDeviceType,
        rust::Str inputDeviceName,
        rust::Str outputDeviceName)
    {
        if (auto* device = audioIoDeviceType.createDevice (
                static_cast<std::string> (inputDeviceName),
                static_cast<std::string> (outputDeviceName)))
        {
            return std::unique_ptr<juce::AudioIODevice> (device);
        }

        return nullptr;
    }
} // namespace audio_io_device_type

namespace audio_io_device
{
    rust::Str getDeviceName (const juce::AudioIODevice& audioIoDevice)
    {
        return toStr (audioIoDevice.getName());
    }

    rust::Str getDeviceTypeName (const juce::AudioIODevice& audioIoDevice)
    {
        return toStr (audioIoDevice.getTypeName());
    }

    rust::Vec<rust::f64> getAvailableSampleRates (juce::AudioIODevice& audioIoDevice)
    {
        const auto sampleRates = audioIoDevice.getAvailableSampleRates();

        rust::Vec<rust::f64> result;
        result.reserve (static_cast<size_t> (sampleRates.size()));
        std::copy (
            std::begin (sampleRates),
            std::end (sampleRates),
            std::back_inserter (result));
        return result;
    }

    rust::Vec<size_t> getAvailableBufferSizes (juce::AudioIODevice& audioIoDevice)
    {
        const auto bufferSizes = audioIoDevice.getAvailableBufferSizes();

        rust::Vec<rust::usize> result;
        result.reserve (static_cast<size_t> (bufferSizes.size()));
        std::copy (
            std::begin (bufferSizes),
            std::end (bufferSizes),
            std::back_inserter (result));
        return result;
    }

    void open (juce::AudioIODevice& audioIoDevice,
               double sampleRate,
               size_t bufferSize)
    {
        audioIoDevice.open (juce::BigInteger {},
                            juce::BigInteger {},
                            sampleRate,
                            static_cast<int> (bufferSize));
    }

    rust::i32 countActiveInputChannels (const juce::AudioIODevice& audioIoDevice)
    {
        return audioIoDevice.getActiveInputChannels().countNumberOfSetBits();
    }

    rust::i32 countActiveOutputChannels (const juce::AudioIODevice& audioIoDevice)
    {
        return audioIoDevice.getActiveOutputChannels().countNumberOfSetBits();
    }
} // namespace audio_io_device

namespace system_audio_volume
{
    void setMuted (bool muted)
    {
        juce::SystemAudioVolume::setMuted (muted);
    }

    bool isMuted()
    {
        return juce::SystemAudioVolume::isMuted();
    }

    void setGain (rust::f32 gain)
    {
        juce::SystemAudioVolume::setGain (gain);
    }

    rust::f32 getGain()
    {
        return juce::SystemAudioVolume::getGain();
    }
} // namespace system_audio_volume

namespace iir_filter
{
    std::unique_ptr<juce::SingleThreadedIIRFilter> createIIRFilter (std::array<rust::f32, 5> coefficients)
    {
        auto filter = std::make_unique<juce::SingleThreadedIIRFilter>();

        juce::IIRCoefficients coeffs;
        coeffs.coefficients[0] = coefficients[0];
        coeffs.coefficients[1] = coefficients[1];
        coeffs.coefficients[2] = coefficients[2];
        coeffs.coefficients[3] = coefficients[3];
        coeffs.coefficients[4] = coefficients[4];

        filter->setCoefficients (coeffs);

        return filter;
    }

    std::array<rust::f32, 5> makeLowPass (double sampleRate,
                                          double cutoffFrequency,
                                          double q)
    {
        const auto coefficients = juce::IIRCoefficients::makeLowPass (
            sampleRate,
            cutoffFrequency,
            q);

        return { coefficients.coefficients[0],
                 coefficients.coefficients[1],
                 coefficients.coefficients[2],
                 coefficients.coefficients[3],
                 coefficients.coefficients[4] };
    }

    std::array<rust::f32, 5> makeHighPass (double sampleRate,
                                           double cutoffFrequency,
                                           double q)
    {
        const auto coefficients = juce::IIRCoefficients::makeHighPass (
            sampleRate,
            cutoffFrequency,
            q);

        return { coefficients.coefficients[0],
                 coefficients.coefficients[1],
                 coefficients.coefficients[2],
                 coefficients.coefficients[3],
                 coefficients.coefficients[4] };
    }

    std::array<rust::f32, 5> makeNotchFilter (double sampleRate,
                                              double cutoffFrequency,
                                              double q)
    {
        const auto coefficients = juce::IIRCoefficients::makeNotchFilter (
            sampleRate,
            cutoffFrequency,
            q);

        return { coefficients.coefficients[0],
                 coefficients.coefficients[1],
                 coefficients.coefficients[2],
                 coefficients.coefficients[3],
                 coefficients.coefficients[4] };
    }
} // namespace iir_filter
} // namespace cxx_juce