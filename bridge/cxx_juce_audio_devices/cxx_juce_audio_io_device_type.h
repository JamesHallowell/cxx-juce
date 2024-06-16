#pragma once

#include <juce_audio_devices/juce_audio_devices.h>
#include <rust/cxx.h>

namespace cxx_juce::audio_io_device_type
{

rust::String getTypeName (const juce::AudioIODeviceType& audioIoDeviceType);
rust::Vec<rust::String> getInputDeviceNames (const juce::AudioIODeviceType& audioIoDeviceType);
rust::Vec<rust::String> getOutputDeviceNames (const juce::AudioIODeviceType& audioIoDeviceType);
std::unique_ptr<juce::AudioIODevice> createDevice (juce::AudioIODeviceType& audioIoDeviceType, rust::Str inputDeviceName, rust::Str outputDeviceName);

} // namespace cxx_juce::audio_io_device_type
