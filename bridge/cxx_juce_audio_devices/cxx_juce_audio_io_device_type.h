#pragma once

#include <juce_audio_devices/juce_audio_devices.h>

namespace cxx_juce
{

class BoxDynAudioIODeviceType
{
    using FatPtr = std::array<std::uintptr_t, 2>;

public:
    BoxDynAudioIODeviceType (BoxDynAudioIODeviceType&& other) noexcept;
    ~BoxDynAudioIODeviceType() noexcept;
    using IsRelocatable = std::true_type;

private:
    FatPtr _repr;
};

std::unique_ptr<juce::AudioIODeviceType> wrapAudioDeviceType (BoxDynAudioIODeviceType deviceType);
} // namespace cxx_juce
