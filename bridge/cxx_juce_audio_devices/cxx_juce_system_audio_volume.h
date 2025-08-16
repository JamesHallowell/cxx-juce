#pragma once

#include <rust/cxx.h>

namespace cxx_juce::system_audio_volume
{

void setMuted (bool muted);
bool isMuted();
void setGain (rust::f32 gain);
rust::f32 getGain();

} // namespace cxx_juce::system_audio_volume
