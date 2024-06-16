#include <cxx_juce_bindings.h>

namespace cxx_juce::audio_io_device
{
rust::Str getDeviceName (const juce::AudioIODevice& audioIoDevice)
{
    return toStr (audioIoDevice.getName());
}

rust::Str getDeviceTypeName (const juce::AudioIODevice& audioIoDevice)
{
    return toStr (audioIoDevice.getTypeName());
}

rust::Vec<rust::f64> getAvailableSampleRates (juce::AudioIODevice& audioIoDevice)
{
    const auto sampleRates = audioIoDevice.getAvailableSampleRates();

    rust::Vec<rust::f64> result;
    result.reserve (static_cast<size_t> (sampleRates.size()));
    std::copy (
        std::begin (sampleRates),
        std::end (sampleRates),
        std::back_inserter (result));
    return result;
}

rust::Vec<size_t> getAvailableBufferSizes (juce::AudioIODevice& audioIoDevice)
{
    const auto bufferSizes = audioIoDevice.getAvailableBufferSizes();

    rust::Vec<rust::usize> result;
    result.reserve (static_cast<size_t> (bufferSizes.size()));
    std::copy (
        std::begin (bufferSizes),
        std::end (bufferSizes),
        std::back_inserter (result));
    return result;
}

void open (juce::AudioIODevice& audioIoDevice,
           double sampleRate,
           size_t bufferSize)
{
    audioIoDevice.open (juce::BigInteger {},
                        juce::BigInteger {},
                        sampleRate,
                        static_cast<int> (bufferSize));
}

rust::i32 countActiveInputChannels (const juce::AudioIODevice& audioIoDevice)
{
    return audioIoDevice.getActiveInputChannels().countNumberOfSetBits();
}

rust::i32 countActiveOutputChannels (const juce::AudioIODevice& audioIoDevice)
{
    return audioIoDevice.getActiveOutputChannels().countNumberOfSetBits();
}
} // namespace cxx_juce::audio_io_device
