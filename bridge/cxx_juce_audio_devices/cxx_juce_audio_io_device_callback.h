#pragma once

#include <juce_audio_devices/juce_audio_devices.h>

#include <cxx_juce_utils.h>

namespace cxx_juce
{

struct DropBoxDynAudioDeviceCallback
{
    void operator() (FatPtr<DropBoxDynAudioDeviceCallback>* callback) const;
};

using BoxDynAudioDeviceCallback = FatPtr<DropBoxDynAudioDeviceCallback>;

std::unique_ptr<juce::AudioIODeviceCallback> wrapAudioDeviceCallback (BoxDynAudioDeviceCallback callback);

} // namespace cxx_juce
