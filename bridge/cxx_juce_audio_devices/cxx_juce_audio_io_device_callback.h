#pragma once

#include <juce_audio_devices/juce_audio_devices.h>

#include <cxx_juce_utils.h>

namespace cxx_juce
{
CXX_JUCE_DECLARE_BOXED_TRAIT_TYPE (AudioDeviceCallback, juce::AudioIODeviceCallback)
} // namespace cxx_juce
