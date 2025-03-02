#pragma once

#include "cxx_juce_audio_callback_wrapper.h"
#include "cxx_juce_audio_device_setup.h"

#include <juce_audio_devices/juce_audio_devices.h>
#include <rust/cxx.h>

namespace cxx_juce
{

struct BoxedAudioIODeviceType;

struct AudioDeviceManager
{
    void initialiseWithDefaultDevices (rust::i32 inputChannels,
                                       rust::i32 outputChannels);
    [[nodiscard]] std::unique_ptr<AudioDeviceSetup> getAudioDeviceSetup() const;
    void setAudioDeviceSetup (const AudioDeviceSetup& setup);
    void addAudioCallback (const std::unique_ptr<AudioCallbackWrapper>& callback);
    void removeAudioCallback (const std::unique_ptr<AudioCallbackWrapper>& callback);
    void addAudioDeviceType (rust::Box<BoxedAudioIODeviceType> audioIODeviceType);
    void setCurrentAudioDeviceType (rust::Str audioDeviceTypeName);
    void playTestSound();
    juce::AudioIODevice* getCurrentAudioDevice();
    const juce::OwnedArray<juce::AudioIODeviceType>& getAvailableDeviceTypes();
    juce::AudioIODeviceType* getCurrentDeviceTypeObject() const;

    juce::AudioDeviceManager _audioDeviceManager;
};

std::unique_ptr<AudioDeviceManager> createAudioDeviceManager();

} // namespace cxx_juce
