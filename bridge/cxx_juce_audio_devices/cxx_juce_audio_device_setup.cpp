#include <cxx_juce_bindings.h>

namespace cxx_juce
{
std::unique_ptr<AudioDeviceSetup> createAudioDeviceSetup()
{
    return std::make_unique<AudioDeviceSetup>();
}

AudioDeviceSetup::AudioDeviceSetup (juce::AudioDeviceManager::AudioDeviceSetup audioDeviceSetup)
    : _audioDeviceSetup (std::move (audioDeviceSetup))
{
}

rust::Str AudioDeviceSetup::outputDeviceName() const
{
    return toStr (_audioDeviceSetup.outputDeviceName);
}

void AudioDeviceSetup::setOutputDeviceName (rust::Str outputDeviceName)
{
    _audioDeviceSetup.outputDeviceName = static_cast<std::string> (outputDeviceName);
}

rust::Str AudioDeviceSetup::inputDeviceName() const
{
    return toStr (_audioDeviceSetup.inputDeviceName);
}

void AudioDeviceSetup::setInputDeviceName (rust::Str inputDeviceName)
{
    _audioDeviceSetup.inputDeviceName = static_cast<std::string> (inputDeviceName);
}

rust::f64 AudioDeviceSetup::sampleRate() const
{
    return _audioDeviceSetup.sampleRate;
}

void AudioDeviceSetup::setSampleRate (rust::f64 sampleRate)
{
    _audioDeviceSetup.sampleRate = sampleRate;
}

rust::i32 AudioDeviceSetup::bufferSize() const
{
    return _audioDeviceSetup.bufferSize;
}

void AudioDeviceSetup::setBufferSize (rust::i32 bufferSize)
{
    _audioDeviceSetup.bufferSize = bufferSize;
}

rust::i32 AudioDeviceSetup::numberOfInputChannels() const
{
    return _audioDeviceSetup.inputChannels.countNumberOfSetBits();
}

void AudioDeviceSetup::setNumberOfInputChannels (rust::i32 numberOfInputChannels)
{
    _audioDeviceSetup.inputChannels.clear();
    _audioDeviceSetup.inputChannels.setRange (0, numberOfInputChannels, true);
}

void AudioDeviceSetup::useDefaultInputChannels (bool useDefaultInputChannels)
{
    _audioDeviceSetup.useDefaultInputChannels = useDefaultInputChannels;
}

bool AudioDeviceSetup::usingDefaultInputChannels() const
{
    return _audioDeviceSetup.useDefaultInputChannels;
}

rust::i32 AudioDeviceSetup::numberOfOutputChannels() const
{
    return _audioDeviceSetup.outputChannels.countNumberOfSetBits();
}

void AudioDeviceSetup::setNumberOfOutputChannels (rust::i32 numberOfOutputChannels)
{
    _audioDeviceSetup.outputChannels.clear();
    _audioDeviceSetup.outputChannels.setRange (0, numberOfOutputChannels, true);
}

void AudioDeviceSetup::useDefaultOutputChannels (bool useDefaultOutputChannels)
{
    _audioDeviceSetup.useDefaultOutputChannels = useDefaultOutputChannels;
}

bool AudioDeviceSetup::usingDefaultOutputChannels() const
{
    return _audioDeviceSetup.useDefaultOutputChannels;
}
} // namespace cxx_juce
