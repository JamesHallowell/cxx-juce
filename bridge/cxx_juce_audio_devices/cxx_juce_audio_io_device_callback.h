#pragma once

#include <juce_audio_devices/juce_audio_devices.h>

namespace cxx_juce
{

class BoxDynAudioDeviceCallback
{
    using FatPtr = std::array<std::uintptr_t, 2>;

public:
    BoxDynAudioDeviceCallback (BoxDynAudioDeviceCallback&& other) noexcept;
    ~BoxDynAudioDeviceCallback() noexcept;
    using IsRelocatable = std::true_type;

private:
    FatPtr _repr;
};

std::unique_ptr<juce::AudioIODeviceCallback> wrapAudioDeviceCallback (BoxDynAudioDeviceCallback callback);

} // namespace cxx_juce
