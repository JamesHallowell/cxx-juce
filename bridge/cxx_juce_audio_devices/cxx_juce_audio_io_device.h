#pragma once

#include <juce_audio_devices/juce_audio_devices.h>
#include <rust/cxx.h>

namespace cxx_juce::audio_io_device
{

rust::Str getDeviceName (const juce::AudioIODevice& audioIoDevice);
rust::Str getDeviceTypeName (const juce::AudioIODevice& audioIoDevice);
rust::Vec<rust::f64> getAvailableSampleRates (juce::AudioIODevice& audioIoDevice);
rust::Vec<size_t> getAvailableBufferSizes (juce::AudioIODevice& audioIoDevice);
void open (juce::AudioIODevice& audioIoDevice, double sampleRate, size_t bufferSize);
rust::i32 countActiveInputChannels (const juce::AudioIODevice& audioIoDevice);
rust::i32 countActiveOutputChannels (const juce::AudioIODevice& audioIoDevice);

} // namespace cxx_juce::audio_io_device
