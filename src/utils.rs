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
pub trait AlignType {
    type Type;
}

#[doc(hidden)]
pub struct Const<const N: usize>;

macro_rules! impl_align {
    ($n:literal, $name:ident) => {
        impl AlignType for Const<$n> {
            type Type = $name;
        }
    };
}

impl_align!(1, u8);
impl_align!(2, u16);
impl_align!(4, u32);
impl_align!(8, u64);

#[derive(Copy, Clone)]
pub(crate) struct PhantomUnsend(core::marker::PhantomData<*mut ()>);

#[doc(hidden)]
#[macro_export]
macro_rules! define_juce_type {
    (
        $(#[$type_attr:meta])* $name:ident,
        layout = $layout:ty,
        $(
            $key:ident = $value:expr
        ),* $(,)?
    ) => {
        #[repr(C)]
        $(#[$type_attr])*
        pub struct $name {
            _space: core::mem::MaybeUninit<[<$crate::utils::Const<{<$layout>::Alignment.repr as usize}> as $crate::utils::AlignType>::Type; <$layout>::Size.repr as usize / <$layout>::Alignment.repr as usize]>,
            _marker: $crate::utils::PhantomUnsend,
        }

        $crate::static_assert_size_and_alignment!($name, $layout);

        $(
            $crate::define_juce_type!(@prop $name, $key, $value);
        )*
    };
    (
        $(#[$type_attr:meta])* $name:ident,
        fields = {
            $(
                $vis:vis $field:ident: $ty:ty = {
                    offset = $field_offset:expr,
                    $(
                        $field_key:ident = $field_value:ident
                    ),* $(,)?
                }
            ),+,
        },
        $(leak_detector = $leak_detector:ty,)*
        layout = $layout:ty,
        $(
            $key:ident = $value:expr
        ),* $(,)?
    ) => {
        #[repr(C)]
        $(#[$type_attr])*
        pub struct $name {
            $(
                $vis $field: $ty,
            )*
            $(
                #[cfg(all(debug_assertions, not(windows)))]
                leak_detector: $leak_detector,
            )*
            _marker: $crate::utils::PhantomUnsend,
        }

        $crate::static_assert_size_and_alignment!($name, $layout);

        $(
            $crate::static_assert_offset_eq!(
                $name,
                $field,
                $field_offset
            );
        )*

        $(
            $crate::define_juce_type!(@prop $name, $key, $value);
        )*

        $(
            impl $name {
                $(
                    $crate::define_juce_type!(@field $name, $field, $ty, $field_key, $field_value);
                )*
            }
        )*
    };
    (@prop $name:ident, cxx_name, $cxx_name:literal) => {
        unsafe impl cxx::ExternType for $name {
            type Id = cxx::type_id!($cxx_name);
            type Kind = cxx::kind::Trivial;
        }
    };
    (@prop $name:ident, drop, $drop:expr) => {
        impl Drop for $name {
            fn drop(&mut self) {
                ($drop)(self);
            }
        }
    };
    (@prop $name:ident, default, $default:expr) => {
        impl Default for $name {
            fn default() -> Self {
                ($default)()
            }
        }
    };
    (@prop $name:ident, equality, $equality:expr) => {
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                ($equality)(self, other)
            }
        }
    };
    (@field $name:ident, $field:ident, $field_ty:ty, with, $with:ident) => {
        pub fn $with(mut self, value: impl Into<$field_ty>) -> Self {
            self.$field = value.into();
            self
        }
    };
    (@field $name:ident, $field:ident, $field_ty:ty, get, $get:ident) => {
        pub fn $get(&self) -> $field_ty {
            self.$field
        }
    };
    (@field $name:ident, $field:ident, $field_ty:ty, get_ref, $get_ref:ident) => {
        pub fn $get_ref(&self) -> &$field_ty {
            &self.$field
        }
    };
    (@field $name:ident, $field:ident, $field_ty:ty, set, $set:ident) => {
        pub fn $set(&mut self, value: impl Into<$field_ty>) {
            self.$field = value.into();
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! define_array_type {
    (
        $name:ident,
        $ty:ty,
        $(
            $key:ident = $value:expr
        ),* $(,)?
    ) => {
        $(
            $crate::define_array_type!(@prop $name, $ty, $key, $value);
        )*
    };
    (@prop $name:ident, $ty:ty, index, $index:expr) => {
        impl $name {
            pub fn get(&self, index: i32) -> $ty {
                ($index)(self, index)
            }
        }
    };
    (@prop $name:ident, $ty:ty, data, $data:expr) => {
        impl AsRef<[$ty]> for $name {
            fn as_ref(&self) -> &[$ty] {
                let data = ($data)(self);
                self.size()
                    .try_into()
                    .map(|size| unsafe { std::slice::from_raw_parts(data, size) })
                    .unwrap_or_default()
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.as_ref())
            }
        }
    };
    (@prop $name:ident, $ty:ty, from_slice, $from_slice:expr) => {
        impl From<&[$ty]> for $name {
            fn from(value: &[$ty]) -> Self {
                let ptr = value.as_ptr();
                let len = value.len();

                len.try_into()
                    .map(|len| unsafe { ($from_slice)(ptr, len) })
                    .unwrap_or_default()
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! define_trait {
    (
        $(#[$trait_attr:meta])* $trait_name:ident $( : $($trait_bounds:path)+ )?,
        $trait_impl_name:ident,
        $cxx_name:literal,
        $(
            $(#[$attr:meta])*
            fn $method_name:ident ( $($args:tt)* ) $(-> $ret:ty)?;
        )*
    ) => {
        $(#[$trait_attr])*
        pub trait $trait_name $( : $($trait_bounds)+ )? {
            $(
                $(#[$attr])*
                fn $method_name( $($args)* ) $(-> $ret)?;
            )*
        }

        unsafe impl cxx::ExternType for Box<dyn $trait_name> {
            type Id = cxx::type_id!($cxx_name);
            type Kind = cxx::kind::Trivial;
        }

        struct $trait_impl_name;

        impl $trait_impl_name {
            fn drop(self_: *mut Box<dyn $trait_name>) {
                unsafe { std::ptr::drop_in_place(self_) };
            }

            $(
                define_trait!(@handle_method
                    $trait_name,
                    $method_name,
                    ( $($args)* ),
                    $(-> $ret)?
                );
            )*
        }
    };
    (@handle_method
        $trait_name:ident,
        $method:ident,
        ( &self $(, $arg:ident : $ty:ty )* $(,)? ),
        $(-> $ret:ty)?
    ) => {
        fn $method(self_: &Box<dyn $trait_name>, $( $arg : $ty ),* ) $(-> $ret)? {
            self_.$method( $( $arg ),* )
        }
    };
    (@handle_method
        $trait_name:ident,
        $method:ident,
        ( &mut self $(, $arg:ident : $ty:ty )* $(,)? ),
        $(-> $ret:ty)?
    ) => {
        fn $method(self_: &mut Box<dyn $trait_name>, $( $arg : $ty ),* ) $(-> $ret)? {
            self_.$method( $( $arg ),* )
        }
    };
}
