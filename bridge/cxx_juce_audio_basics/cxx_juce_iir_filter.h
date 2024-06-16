#pragma once

#include <juce_audio_basics/juce_audio_basics.h>
#include <rust/cxx.h>

namespace cxx_juce::iir_filter
{
std::unique_ptr<juce::SingleThreadedIIRFilter> createIIRFilter (std::array<rust::f32, 5> coefficients);
std::array<rust::f32, 5> makeLowPass (double sampleRate, double cutoffFrequency, double q);
std::array<rust::f32, 5> makeHighPass (double sampleRate, double cutoffFrequency, double q);
std::array<rust::f32, 5> makeNotchFilter (double sampleRate, double cutoffFrequency, double q);
} // namespace cxx_juce::iir_filter
