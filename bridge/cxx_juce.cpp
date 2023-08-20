#include "cxx_juce_bindings.h"

namespace cxx_juce
{
rust::String juceVersion()
{
    return juce::SystemStats::getJUCEVersion().toStdString();
}

void initialiseJuce()
{
    juce::initialiseJuce_GUI();
}

void shutdownJuce()
{
    juce::shutdownJuce_GUI();
}

rust::Str toStr (const juce::String& string)
{
    return { string.toRawUTF8(), string.getNumBytesAsUTF8() };
}
} // namespace cxx_juce