#include <cxx_juce_utils.h>

#include <cxx-juce/src/juce_audio_devices/device_manager.rs.h>

CXX_JUCE_ASSERT_SIZE_ALIGN (AudioDeviceManager::AudioDeviceSetup, cxx_juce::AudioDeviceSetupLayout)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceManager::AudioDeviceSetup, outputDeviceName, cxx_juce::AudioDeviceSetupLayout::OutputDeviceNameOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceManager::AudioDeviceSetup, inputDeviceName, cxx_juce::AudioDeviceSetupLayout::InputDeviceNameOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceManager::AudioDeviceSetup, sampleRate, cxx_juce::AudioDeviceSetupLayout::SampleRateOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceManager::AudioDeviceSetup, bufferSize, cxx_juce::AudioDeviceSetupLayout::BufferSizeOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceManager::AudioDeviceSetup, inputChannels, cxx_juce::AudioDeviceSetupLayout::InputChannelsOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceManager::AudioDeviceSetup, useDefaultInputChannels, cxx_juce::AudioDeviceSetupLayout::UseDefaultInputChannelsOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceManager::AudioDeviceSetup, outputChannels, cxx_juce::AudioDeviceSetupLayout::OutputChannelsOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (AudioDeviceManager::AudioDeviceSetup, useDefaultOutputChannels, cxx_juce::AudioDeviceSetupLayout::UseDefaultOutputChannelsOffset)
