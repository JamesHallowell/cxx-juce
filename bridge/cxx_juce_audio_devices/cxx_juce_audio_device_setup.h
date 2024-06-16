#pragma once

#include <juce_audio_devices/juce_audio_devices.h>
#include <rust/cxx.h>

#include <memory>

namespace cxx_juce
{

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

} // namespace cxx_juce
