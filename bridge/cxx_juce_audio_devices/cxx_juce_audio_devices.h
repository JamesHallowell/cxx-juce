#pragma once

#include <juce_audio_devices/juce_audio_devices.h>

#include <cxx_juce_audio_devices/cxx_juce_audio_io_device.h>
#include <cxx_juce_audio_devices/cxx_juce_audio_io_device_callback.h>
#include <cxx_juce_audio_devices/cxx_juce_audio_io_device_type.h>
#include <cxx_juce_utils.h>

#include <rust/cxx.h>

namespace juce
{
using AudioDeviceSetup = AudioDeviceManager::AudioDeviceSetup;
}

CXX_JUCE_DECLARE_RELOCATABLE (AudioDeviceManager::AudioDeviceSetup)
