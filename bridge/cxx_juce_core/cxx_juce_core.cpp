#include <cxx_juce_utils.h>

#include <cxx-juce/src/juce_core/array.rs.h>
#include <cxx-juce/src/juce_core/bigint.rs.h>
#include <cxx-juce/src/juce_core/memory.rs.h>
#include <cxx-juce/src/juce_core/string.rs.h>
#include <cxx-juce/src/juce_core/time.rs.h>

namespace juce
{
using IntArrayLayout = ArrayLayout;
using DoubleArrayLayout = ArrayLayout;

rust::i64 toMilliseconds (const Time& time)
{
    return static_cast<rust::i64> (time.toMilliseconds());
}
} // namespace juce

CXX_JUCE_ASSERT_SIZE_ALIGN (String)
CXX_JUCE_ASSERT_SIZE_ALIGN (CharPointer_UTF8)
CXX_JUCE_ASSERT_SIZE_ALIGN (BigInteger)
CXX_JUCE_ASSERT_SIZE_ALIGN (IntArray)
CXX_JUCE_ASSERT_SIZE_ALIGN (DoubleArray)
CXX_JUCE_ASSERT_SIZE_ALIGN (StringArray)
CXX_JUCE_ASSERT_SIZE_ALIGN (Time)
