#[doc(hidden)]
#[macro_export]
macro_rules! static_assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        const _: [(); $left] = [(); $right];
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! static_assert_size_and_alignment {
    ($type:ty, $layout:ty) => {
        $crate::static_assert_eq!(size_of::<$type>(), <$layout>::Size.repr as usize);
        $crate::static_assert_eq!(align_of::<$type>(), <$layout>::Alignment.repr as usize);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! static_assert_offset_eq {
    ($type:ty, $field:ident, $offset:expr) => {
        $crate::static_assert_eq!(core::mem::offset_of!($type, $field), $offset.repr as usize);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! define_juce_type {
    ($name:ident, size = $size:expr, align = $align:expr, check_with = $layout:ty) => {
        #[repr(C, align($align))]
        pub struct $name {
            _space: core::mem::MaybeUninit<[u8; $size]>,
        }
        $crate::static_assert_size_and_alignment!($name, $layout);
    };
    ($name:ident, size = $size:expr, align = $align:expr, has_leak_detector, check_with = $layout:ty) => {
        #[repr(C, align($align))]
        pub struct $name {
            _space: core::mem::MaybeUninit<[u8; $size]>,
            #[cfg(all(debug_assertions, not(windows)))]
            _leak_detector: $crate::utils::LeakedObjectDetector,
        }
        $crate::static_assert_size_and_alignment!($name, $layout);
    };
}

define_juce_type!(
    LeakedObjectDetector,
    size = 1,
    align = 1,
    check_with = juce::LeakedObjectDetectorLayout
);

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum LeakedObjectDetectorLayout {
        Size = 1,
        Alignment = 1,
    }
}
