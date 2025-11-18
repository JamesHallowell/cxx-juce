#pragma once

#include <juce_core/juce_core.h>

#include <rust/cxx.h>

namespace juce
{
using IntArray = Array<int>;
using FloatArray = Array<float>;
using DoubleArray = Array<double>;

rust::i64 toMilliseconds (const Time& time);
} // namespace juce

CXX_JUCE_DECLARE_RELOCATABLE (String)
CXX_JUCE_DECLARE_RELOCATABLE (BigInteger)
CXX_JUCE_DECLARE_RELOCATABLE_T (Array<T>)
CXX_JUCE_DECLARE_RELOCATABLE_T (OwnedArray<T>)
CXX_JUCE_DECLARE_RELOCATABLE (StringArray)
CXX_JUCE_DECLARE_RELOCATABLE (File)
CXX_JUCE_DECLARE_RELOCATABLE (FileSearchPath)
