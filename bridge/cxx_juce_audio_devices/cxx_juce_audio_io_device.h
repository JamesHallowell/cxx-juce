#pragma once

#include <juce_audio_devices/juce_audio_devices.h>

namespace cxx_juce
{

class BoxDynAudioDevice
{
    using FatPtr = std::array<std::uintptr_t, 2>;

public:
    BoxDynAudioDevice (BoxDynAudioDevice&& other) noexcept;
    ~BoxDynAudioDevice() noexcept;
    using IsRelocatable = std::true_type;

private:
    FatPtr _repr;
};

std::unique_ptr<juce::AudioIODevice> wrapAudioDevice (BoxDynAudioDevice device);
} // namespace cxx_juce
