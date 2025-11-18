#pragma once

#define CXX_JUCE_ASSERT_SIZE_ALIGN(TYPE, LAYOUT)                                 \
    static_assert (sizeof (juce ::TYPE) == static_cast<size_t> (LAYOUT ::Size)); \
    static_assert (alignof (juce ::TYPE) == static_cast<size_t> (LAYOUT ::Alignment));

#define CXX_JUCE_ASSERT_FIELD_OFFSET(TYPE, FIELD, OFFSET) \
    static_assert (offsetof (juce ::TYPE, FIELD) == static_cast<size_t> (OFFSET));
