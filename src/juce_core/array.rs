use crate::define_juce_type;

macro_rules! define_array {
    (
        $name:ident,
        $ty: ty,
        cxx_name = $cxx_name:literal,
        default = $default:expr,
        constructor = $constructor:expr,
        drop = $drop:expr
    ) => {
        $crate::define_juce_type!($name, size = 16, align = 8, check_with = juce::ArrayLayout);

        unsafe impl cxx::ExternType for $name {
            type Id = cxx::type_id!($cxx_name);
            type Kind = cxx::kind::Trivial;
        }

        impl Default for $name {
            fn default() -> Self {
                $default()
            }
        }

        impl From<&[$ty]> for $name {
            fn from(value: &[$ty]) -> Self {
                let ptr = value.as_ptr();
                let len = value.len();

                len.try_into()
                    .map(|len| unsafe { $constructor(ptr, len) })
                    .unwrap_or_default()
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                $drop(self);
            }
        }

        impl AsRef<[$ty]> for $name {
            fn as_ref(&self) -> &[$ty] {
                let data = self.get_raw_data_pointer();
                self.size()
                    .try_into()
                    .map(|size| unsafe { std::slice::from_raw_parts(data, size) })
                    .unwrap_or_default()
            }
        }
    };
}

define_array!(
    IntArray,
    i32,
    cxx_name = "juce::IntArray",
    default = juce::construct_i32_array,
    constructor = juce::construct_i32_array_range,
    drop = juce::drop_i32_array
);

define_array!(
    FloatArray,
    f32,
    cxx_name = "juce::FloatArray",
    default = juce::construct_f32_array,
    constructor = juce::construct_f32_array_range,
    drop = juce::drop_f32_array
);

define_array!(
    DoubleArray,
    f64,
    cxx_name = "juce::DoubleArray",
    default = juce::construct_f64_array,
    constructor = juce::construct_f64_array_range,
    drop = juce::drop_f64_array
);

define_juce_type!(
    StringArray,
    size = 16,
    align = 8,
    has_leak_detector,
    check_with = juce::StringArrayLayout
);

unsafe impl cxx::ExternType for StringArray {
    type Id = cxx::type_id!("juce::StringArray");
    type Kind = cxx::kind::Trivial;
}

impl Drop for StringArray {
    fn drop(&mut self) {
        juce::drop_string_array(self);
    }
}

impl Default for StringArray {
    fn default() -> Self {
        juce::construct_string_array()
    }
}

pub struct StringArrayIter<'a> {
    array: &'a StringArray,
    index: i32,
}

impl<'a> Iterator for StringArrayIter<'a> {
    type Item = &'a juce::JuceString;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        if index < self.array.size() {
            self.index += 1;
            Some(self.array.get_reference(index))
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a StringArray {
    type Item = &'a juce::JuceString;
    type IntoIter = StringArrayIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        StringArrayIter {
            array: self,
            index: 0,
        }
    }
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum StringArrayLayout {
        #[cfg(all(debug_assertions, not(windows)))]
        Size = 24,
        #[cfg(any(not(debug_assertions), windows))]
        Size = 16,
        Alignment = 8,
    }

    enum ArrayLayout {
        Size = 16,
        Alignment = 8,
    }

    unsafe extern "C++" {
        include!("cxx_juce.h");
        include!("cxx_juce_core/cxx_juce_core.h");

        #[cxx_name = "String"]
        type JuceString = crate::juce_core::JuceString;

        type IntArray = super::IntArray;

        #[namespace = "cxx_juce"]
        #[rust_name = "construct_i32_array"]
        fn construct() -> IntArray;

        #[namespace = "cxx_juce"]
        #[rust_name = "construct_i32_array_range"]
        unsafe fn construct(ptr: *const i32, size: i32) -> IntArray;

        #[namespace = "cxx_juce"]
        #[rust_name = "drop_i32_array"]
        fn drop(value: &mut IntArray);

        #[cxx_name = "getRawDataPointer"]
        fn get_raw_data_pointer(self: &IntArray) -> *const i32;

        fn size(self: &IntArray) -> i32;

        type FloatArray = super::FloatArray;

        #[namespace = "cxx_juce"]
        #[rust_name = "construct_f32_array"]
        fn construct() -> FloatArray;

        #[namespace = "cxx_juce"]
        #[rust_name = "construct_f32_array_range"]
        unsafe fn construct(ptr: *const f32, size: i32) -> FloatArray;

        #[namespace = "cxx_juce"]
        #[rust_name = "drop_f32_array"]
        fn drop(value: &mut FloatArray);

        #[cxx_name = "getRawDataPointer"]
        fn get_raw_data_pointer(self: &FloatArray) -> *const f32;

        fn size(self: &FloatArray) -> i32;

        type DoubleArray = super::DoubleArray;

        #[namespace = "cxx_juce"]
        #[rust_name = "construct_f64_array"]
        fn construct() -> DoubleArray;

        #[namespace = "cxx_juce"]
        #[rust_name = "construct_f64_array_range"]
        unsafe fn construct(ptr: *const f64, size: i32) -> DoubleArray;

        #[namespace = "cxx_juce"]
        #[rust_name = "drop_f64_array"]
        fn drop(value: &mut DoubleArray);

        #[cxx_name = "getRawDataPointer"]
        fn get_raw_data_pointer(self: &DoubleArray) -> *const f64;

        fn size(self: &DoubleArray) -> i32;

        type StringArray = super::StringArray;

        #[namespace = "cxx_juce"]
        #[rust_name = "construct_string_array"]
        fn construct() -> StringArray;

        #[namespace = "cxx_juce"]
        #[rust_name = "drop_string_array"]
        fn drop(value: &mut StringArray);

        fn add(self: &mut StringArray, value: JuceString);

        #[cxx_name = "getReference"]
        fn get_reference(self: &StringArray, index: i32) -> &JuceString;

        fn size(self: &StringArray) -> i32;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::juce_core::JuceString;

    #[test]
    fn string_array_iter() {
        let mut array = StringArray::default();
        array.add(JuceString::new("Hello"));
        array.add(JuceString::new("World"));
        array.add(JuceString::new("!"));

        assert_eq!(array.size(), 3);

        let mut iter = array.into_iter();
        assert_eq!(iter.next(), Some(&JuceString::new("Hello")));
        assert_eq!(iter.next(), Some(&JuceString::new("World")));
        assert_eq!(iter.next(), Some(&JuceString::new("!")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn array_from_slice() {
        let array = [1, 2, 3];
        let array = juce::IntArray::from(array.as_slice());

        assert_eq!(array.size(), 3);
        assert_eq!(array.as_ref(), &[1, 2, 3]);
    }
}
