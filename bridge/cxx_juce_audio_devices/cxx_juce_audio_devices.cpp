#include <cxx_juce_utils.h>

#include <cxx-juce/src/juce_audio_devices/device_manager.rs.h>

CXX_JUCE_ASSERT_SIZE_ALIGN (AudioDeviceSetup)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceSetup, outputDeviceName, OutputDeviceNameOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceSetup, inputDeviceName, InputDeviceNameOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceSetup, sampleRate, SampleRateOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceSetup, bufferSize, BufferSizeOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceSetup, inputChannels, InputChannelsOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceSetup, useDefaultInputChannels, UseDefaultInputChannelsOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceSetup, outputChannels, OutputChannelsOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceSetup, useDefaultOutputChannels, UseDefaultOutputChannelsOffset)
