#pragma once

#include <juce_audio_basics/juce_audio_basics.h>

#include <rust/cxx.h>

template <>
struct rust::IsRelocatable<juce::IIRCoefficients> : std::true_type
{
};

template <>
struct rust::IsRelocatable<juce::SingleThreadedIIRFilter> : std::true_type
{
};
