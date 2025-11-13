#pragma once

#include <juce_audio_devices/juce_audio_devices.h>

#include <cxx_juce_audio_devices/cxx_juce_audio_io_device.h>
#include <cxx_juce_audio_devices/cxx_juce_audio_io_device_callback.h>
#include <cxx_juce_audio_devices/cxx_juce_audio_io_device_type.h>

#include <rust/cxx.h>

template <>
struct rust::IsRelocatable<juce::AudioDeviceManager::AudioDeviceSetup> : std::true_type
{
};
