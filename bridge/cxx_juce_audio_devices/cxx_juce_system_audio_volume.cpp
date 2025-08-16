#include <cxx_juce_bindings.h>

namespace cxx_juce::system_audio_volume
{
void setMuted (bool muted)
{
    juce::SystemAudioVolume::setMuted (muted);
}

bool isMuted()
{
    return juce::SystemAudioVolume::isMuted();
}

void setGain (rust::f32 gain)
{
    juce::SystemAudioVolume::setGain (gain);
}

rust::f32 getGain()
{
    return juce::SystemAudioVolume::getGain();
}
} // namespace cxx_juce::system_audio_volume

namespace cxx_juce
{
void wowee(const Wowza& wowza)
{
    wowza.do_do();
    auto& x = wowza.get();
}

void Shared::cool() const noexcept
{
}

void Shared::cool2() noexcept {

}

Shared makeShared()
{
    return Shared {
        std::make_unique<AudioDeviceManager>()
    };
}
}
