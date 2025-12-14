#pragma once

#include <juce_audio_processors/juce_audio_processors.h>

#include <cxx_juce_utils.h>
#include <rust/cxx.h>

namespace juce
{
using OwnedArrayPluginDescription = OwnedArray<PluginDescription>;
} // namespace juce

namespace cxx_juce
{
juce::String audioProcessorGetName(const juce::AudioProcessor& processor) noexcept;

CXX_JUCE_DECLARE_BOXED_TRAIT_TYPE(AudioPluginFormat, juce::AudioPluginFormat)
CXX_JUCE_DECLARE_BOXED_TRAIT_TYPE(AudioPlugin, juce::AudioPluginInstance)
} // namespace cxx_juce

CXX_JUCE_DECLARE_RELOCATABLE(AudioPluginFormatManager)
CXX_JUCE_DECLARE_RELOCATABLE(PluginDescription)
