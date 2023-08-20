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

rust::String juceVersion();

void initialiseJuce();
void shutdownJuce();

rust::Str toStr (const juce::String& string);

struct AudioDeviceSetup
{
    AudioDeviceSetup() = default;
    explicit AudioDeviceSetup (juce::AudioDeviceManager::AudioDeviceSetup audioDeviceSetup);

    [[nodiscard]] rust::Str outputDeviceName() const;
    void setOutputDeviceName (rust::Str outputDeviceName);
    [[nodiscard]] rust::Str inputDeviceName() const;
    void setInputDeviceName (rust::Str inputDeviceName);
    [[nodiscard]] rust::f64 sampleRate() const;
    void setSampleRate (rust::f64 sampleRate);
    [[nodiscard]] rust::i32 bufferSize() const;
    void setBufferSize (rust::i32 bufferSize);
    [[nodiscard]] rust::i32 numberOfInputChannels() const;
    void setNumberOfInputChannels (rust::i32 numberOfInputChannels);
    void useDefaultInputChannels (bool useDefaultInputChannels);
    [[nodiscard]] bool usingDefaultInputChannels() const;
    [[nodiscard]] rust::i32 numberOfOutputChannels() const;
    void setNumberOfOutputChannels (rust::i32 numberOfOutputChannels);
    void useDefaultOutputChannels (bool useDefaultOutputChannels);
    [[nodiscard]] bool usingDefaultOutputChannels() const;

    juce::AudioDeviceManager::AudioDeviceSetup _audioDeviceSetup;
};

std::unique_ptr<AudioDeviceSetup> createAudioDeviceSetup();

class AudioCallbackHandle : public juce::AudioIODeviceCallback
{
public:
    explicit AudioCallbackHandle (juce::AudioDeviceManager& audioDeviceManager,
                                  rust::Box<BoxedAudioIODeviceCallback> callback);
    ~AudioCallbackHandle() override;

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

struct AudioDeviceManager
{
    void initialiseWithDefaultDevices (rust::i32 inputChannels,
                                       rust::i32 outputChannels);
    [[nodiscard]] std::unique_ptr<AudioDeviceSetup> getAudioDeviceSetup() const;
    void setAudioDeviceSetup (const AudioDeviceSetup& setup);
    [[nodiscard]] std::unique_ptr<AudioCallbackHandle>
        addAudioCallback (rust::Box<BoxedAudioIODeviceCallback> callback);
    void addAudioDeviceType (rust::Box<BoxedAudioIODeviceType> audioIODeviceType);
    void setCurrentAudioDeviceType (rust::Str audioDeviceTypeName);
    void playTestSound();
    juce::AudioIODevice* getCurrentAudioDevice() const;
    const juce::OwnedArray<juce::AudioIODeviceType>& getAvailableDeviceTypes();
    juce::AudioIODeviceType* getCurrentDeviceTypeObject() const;

    juce::AudioDeviceManager _audioDeviceManager;
};

std::unique_ptr<AudioDeviceManager> createAudioDeviceManager();

namespace audio_io_device_type
{
    rust::String getTypeName (const juce::AudioIODeviceType& audioIoDeviceType);
    rust::Vec<rust::String> getInputDeviceNames (const juce::AudioIODeviceType& audioIoDeviceType);
    rust::Vec<rust::String> getOutputDeviceNames (const juce::AudioIODeviceType& audioIoDeviceType);
    std::unique_ptr<juce::AudioIODevice> createDevice (juce::AudioIODeviceType& audioIoDeviceType, rust::Str inputDeviceName, rust::Str outputDeviceName);
} // namespace audio_io_device_type

namespace audio_io_device
{
    rust::Str getDeviceName (const juce::AudioIODevice& audioIoDevice);
    rust::Str getDeviceTypeName (const juce::AudioIODevice& audioIoDevice);
    rust::Vec<rust::f64> getAvailableSampleRates (juce::AudioIODevice& audioIoDevice);
    rust::Vec<size_t> getAvailableBufferSizes (juce::AudioIODevice& audioIoDevice);
    void open (juce::AudioIODevice& audioIoDevice, double sampleRate, size_t bufferSize);
    rust::i32 countActiveInputChannels (const juce::AudioIODevice& audioIoDevice);
    rust::i32 countActiveOutputChannels (const juce::AudioIODevice& audioIoDevice);
} // namespace audio_io_device

namespace system_audio_volume
{
    void setMuted (bool muted);
    bool isMuted();
    void setGain (rust::f32 gain);
    rust::f32 getGain();
} // namespace system_audio_volume

namespace iir_filter
{
    std::unique_ptr<juce::SingleThreadedIIRFilter> createIIRFilter (std::array<rust::f32, 5> coefficients);
    std::array<rust::f32, 5> makeLowPass (double sampleRate, double cutoffFrequency, double q);
    std::array<rust::f32, 5> makeHighPass (double sampleRate, double cutoffFrequency, double q);
    std::array<rust::f32, 5> makeNotchFilter (double sampleRate, double cutoffFrequency, double q);
} // namespace iir_filter

namespace message_manager
{
    juce::MessageManager* getInstanceWithoutCreating();
}

} // namespace cxx_juce