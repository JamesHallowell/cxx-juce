#pragma once

#include <juce_audio_devices/juce_audio_devices.h>

#include <cxx_juce_utils.h>

namespace cxx_juce
{
struct DropBoxDynAudioDevice
{
    void operator() (FatPtr<DropBoxDynAudioDevice>* device) const;
};

using BoxDynAudioDevice = FatPtr<DropBoxDynAudioDevice>;

std::unique_ptr<juce::AudioIODevice> wrapAudioDevice (BoxDynAudioDevice device);
} // namespace cxx_juce
