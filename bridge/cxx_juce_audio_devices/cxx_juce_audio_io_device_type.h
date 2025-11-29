#pragma once

#include <juce_audio_devices/juce_audio_devices.h>

#include <cxx_juce_utils.h>

namespace cxx_juce
{

struct DropBoxDynAudioIODeviceType
{
    void operator() (FatPtr<DropBoxDynAudioIODeviceType>* deviceType) const;
};

using BoxDynAudioIODeviceType = FatPtr<DropBoxDynAudioIODeviceType>;

std::unique_ptr<juce::AudioIODeviceType> wrapAudioDeviceType (BoxDynAudioIODeviceType deviceType);
} // namespace cxx_juce
