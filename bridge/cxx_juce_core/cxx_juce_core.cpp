#include <cxx_juce_utils.h>

#include <cxx-juce/src/juce_core/array.rs.h>
#include <cxx-juce/src/juce_core/bigint.rs.h>
#include <cxx-juce/src/juce_core/string.rs.h>
#include <cxx-juce/src/utils.rs.h>

CXX_JUCE_ASSERT_SIZE_ALIGN (String, juce::StringLayout)
CXX_JUCE_ASSERT_SIZE_ALIGN (CharPointer_UTF8, juce::CharPointerUTF8Layout)
CXX_JUCE_ASSERT_SIZE_ALIGN (BigInteger, juce::BigIntegerLayout)
CXX_JUCE_ASSERT_SIZE_ALIGN (IntArray, juce::ArrayLayout)
CXX_JUCE_ASSERT_SIZE_ALIGN (DoubleArray, juce::ArrayLayout)
CXX_JUCE_ASSERT_SIZE_ALIGN (StringArray, juce::StringArrayLayout)
CXX_JUCE_ASSERT_SIZE_ALIGN (LeakedObjectDetector<void>, juce::LeakedObjectDetectorLayout)
