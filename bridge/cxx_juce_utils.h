#pragma once

#include <juce_core/system/juce_CompilerWarnings.h>
#include <rust/cxx.h>

#include <array>

#define CXX_JUCE_ASSERT_SIZE_ALIGN_WITH(TYPE, LAYOUT)                                   \
    static_assert (sizeof (juce ::TYPE) == static_cast<size_t> (juce ::LAYOUT ::Size)); \
    static_assert (alignof (juce ::TYPE) == static_cast<size_t> (juce ::LAYOUT ::Alignment));

#define CXX_JUCE_ASSERT_SIZE_ALIGN(TYPE) \
    CXX_JUCE_ASSERT_SIZE_ALIGN_WITH (TYPE, TYPE##Layout)

#define CXX_JUCE_ASSERT_FIELD_OFFSET(TYPE, FIELD, OFFSET)                                                \
    JUCE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Winvalid-offsetof")                                           \
    static_assert (offsetof (juce ::TYPE, FIELD) == static_cast<size_t> (juce ::TYPE##Layout ::OFFSET)); \
    JUCE_END_IGNORE_WARNINGS_GCC_LIKE

#define CXX_JUCE_DECLARE_RELOCATABLE(TYPE)                   \
    template <>                                              \
    struct rust::IsRelocatable<juce ::TYPE> : std::true_type \
    {                                                        \
    };

#define CXX_JUCE_DECLARE_RELOCATABLE_T(TYPE)                 \
    template <typename T>                                    \
    struct rust::IsRelocatable<juce ::TYPE> : std::true_type \
    {                                                        \
    };

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

#define CXX_JUCE_DECLARE_BOXED_TRAIT_TYPE_CUSTOM(TRAIT)            \
    struct Drop##TRAIT                                             \
    {                                                              \
        void operator() (FatPtr<Drop##TRAIT>* ptr) const noexcept; \
    };                                                             \
    using BoxDyn##TRAIT = FatPtr<Drop##TRAIT>;

#define CXX_JUCE_DECLARE_BOXED_TRAIT_TYPE(TRAIT, CLASS)            \
    struct Drop##TRAIT                                             \
    {                                                              \
        void operator() (FatPtr<Drop##TRAIT>* ptr) const noexcept; \
    };                                                             \
    using BoxDyn##TRAIT = FatPtr<Drop##TRAIT>;                     \
    std::unique_ptr<CLASS> wrap (BoxDyn##TRAIT) noexcept;

#define CXX_JUCE_DEFINE_BOXED_TRAIT_TYPE(TRAIT)                             \
    void Drop##TRAIT ::operator() (FatPtr<Drop##TRAIT>* ptr) const noexcept \
    {                                                                       \
        TRAIT##Impl::drop (ptr);                                            \
    }
