#pragma once

#include <juce_core/juce_core.h>

#include <rust/cxx.h>

namespace juce
{
using IntArray = Array<int>;
using FloatArray = Array<float>;
using DoubleArray = Array<double>;
} // namespace juce

template <>
struct rust::IsRelocatable<juce::String> : std::true_type
{
};

template <>
struct rust::IsRelocatable<juce::BigInteger> : std::true_type
{
};

template <typename T>
struct rust::IsRelocatable<juce::Array<T>> : std::true_type
{
};

template <>
struct rust::IsRelocatable<juce::StringArray> : std::true_type
{
};
