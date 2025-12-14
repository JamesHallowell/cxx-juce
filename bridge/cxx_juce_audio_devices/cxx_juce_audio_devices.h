#pragma once

#include <juce_audio_devices/juce_audio_devices.h>

#include <cxx_juce_utils.h>
#include <rust/cxx.h>

namespace juce
{
using AudioIODeviceTypeArray = OwnedArray<AudioIODeviceType>;
using AudioDeviceSetup = AudioDeviceManager::AudioDeviceSetup;
using MidiDeviceInfoArray = Array<MidiDeviceInfo>;
} // namespace juce

namespace cxx_juce
{
CXX_JUCE_DECLARE_BOXED_TRAIT_TYPE(AudioDevice, juce::AudioIODevice)
CXX_JUCE_DECLARE_BOXED_TRAIT_TYPE(AudioDeviceCallback, juce::AudioIODeviceCallback)
CXX_JUCE_DECLARE_BOXED_TRAIT_TYPE(AudioDeviceType, juce::AudioIODeviceType)
CXX_JUCE_DECLARE_BOXED_TRAIT_TYPE(MidiInputCallback, juce::MidiInputCallback)
} // namespace cxx_juce

CXX_JUCE_DECLARE_RELOCATABLE(AudioDeviceManager::AudioDeviceSetup)
CXX_JUCE_DECLARE_RELOCATABLE(MidiDeviceInfo)
