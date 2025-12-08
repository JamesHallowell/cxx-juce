#pragma once

#include <cxx_juce_audio_basics/cxx_juce_audio_basics.h>
#include <cxx_juce_audio_devices/cxx_juce_audio_devices.h>
#include <cxx_juce_audio_processors/cxx_juce_audio_processors.h>
#include <cxx_juce_core/cxx_juce_core.h>
#include <cxx_juce_events/cxx_juce_events.h>

#include <concepts>
#include <memory>

#include <rust/cxx.h>

namespace cxx_juce
{
inline rust::Str toStr (const juce::String& string)
{
    return { string.toRawUTF8(), string.getNumBytesAsUTF8() };
}

template <typename T, typename... Args>
T construct (Args... args)
{
    return T { std::forward<Args> (args)... };
}

template <typename T, typename... Args>
std::unique_ptr<T> makeUnique (Args... args)
{
    return std::make_unique<T> (std::forward<Args> (args)...);
}

template <typename T>
void drop (T& value)
{
    value.~T();
}

template <typename T>
auto eq (const T& a, const T& b)
{
    return a == b;
}

template <typename T, typename I>
auto index (const T& container, I index)
{
    return container[index];
}

template <typename T, typename U>
requires std::derived_from<T, U> const U& derivedCast (const T& value)
{
    return value;
}

template <typename T, typename U>
requires std::derived_from<T, U> U& derivedCastMut (T& value)
{
    return value;
}

} // namespace cxx_juce
