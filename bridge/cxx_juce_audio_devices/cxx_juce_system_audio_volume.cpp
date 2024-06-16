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
