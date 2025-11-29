#pragma once

#include <rust/cxx.h>

#include <array>

#define CXX_JUCE_ASSERT_SIZE_ALIGN(TYPE, LAYOUT)                                 \
    static_assert (sizeof (juce ::TYPE) == static_cast<size_t> (LAYOUT ::Size)); \
    static_assert (alignof (juce ::TYPE) == static_cast<size_t> (LAYOUT ::Alignment));

#define CXX_JUCE_ASSERT_FIELD_OFFSET(TYPE, FIELD, OFFSET) \
    static_assert (offsetof (juce ::TYPE, FIELD) == static_cast<size_t> (OFFSET));

namespace cxx_juce
{
template <typename Deleter>
class FatPtr
{
    using Repr = std::array<std::uintptr_t, 2>;
    static constexpr auto null = Repr { 0, 0 };

public:
    FatPtr (FatPtr&& other) noexcept
        : _repr { other._repr }
    {
        other._repr = null;
    }

    ~FatPtr() noexcept
    {
        if (_repr != null)
        {
            Deleter {}(this);
        }
    }

private:
    Repr _repr {};
};
} // namespace cxx_juce

template <typename Deleter>
struct rust::IsRelocatable<cxx_juce::FatPtr<Deleter>> : std::true_type
{
};
