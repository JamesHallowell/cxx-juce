use crate::{define_array_type, define_juce_type};

define_juce_type! {
    IntArray,
    layout = ArrayLayout,
    cxx_name = "juce::IntArray",
    default = juce::int_array_new,
    drop = juce::int_array_drop,
}

define_array_type! {
    IntArray,
    i32,
    data = IntArray::data,
    from_slice = juce::int_array_new_from_slice,
}

define_juce_type! {
    FloatArray,
    layout = ArrayLayout,
    cxx_name = "juce::FloatArray",
    default = juce::float_array_new,
    drop = juce::float_array_drop,
}

define_array_type! {
    FloatArray,
    f32,
    data = FloatArray::data,
    from_slice = juce::float_array_new_from_slice,
}

define_juce_type! {
    DoubleArray,
    layout = ArrayLayout,
    cxx_name = "juce::DoubleArray",
    default = juce::double_array_new,
    drop = juce::double_array_drop,
}

define_array_type! {
    DoubleArray,
    f64,
    data = DoubleArray::data,
    from_slice = juce::double_array_new_from_slice,
}

define_juce_type! {
    StringArray,
    layout = juce::StringArrayLayout,
    cxx_name = "juce::StringArray",
    drop = juce::string_array_drop,
    default = juce::string_array_new,
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

pub(crate) use juce::ArrayLayout;

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum ArrayLayout {
        Size = 16,
        Alignment = 8,
    }

    enum StringArrayLayout {
        #[cfg(all(debug_assertions, not(windows)))]
        Size = 24,
        #[cfg(any(not(debug_assertions), windows))]
        Size = 16,
        Alignment = 8,
    }

    unsafe extern "C++" {
        include!("cxx_juce.h");
        include!("cxx_juce_core/cxx_juce_core.h");

        type JuceString = crate::juce_core::JuceString;
        type IntArray = super::IntArray;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn int_array_new() -> IntArray;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        unsafe fn int_array_new_from_slice(ptr: *const i32, size: i32) -> IntArray;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn int_array_drop(value: &mut IntArray);

        #[cxx_name = "getRawDataPointer"]
        fn data(self: &IntArray) -> *const i32;

        fn size(self: &IntArray) -> i32;

        type FloatArray = super::FloatArray;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn float_array_new() -> FloatArray;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        unsafe fn float_array_new_from_slice(ptr: *const f32, size: i32) -> FloatArray;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn float_array_drop(value: &mut FloatArray);

        #[cxx_name = "getRawDataPointer"]
        fn data(self: &FloatArray) -> *const f32;

        fn size(self: &FloatArray) -> i32;

        type DoubleArray = super::DoubleArray;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn double_array_new() -> DoubleArray;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        unsafe fn double_array_new_from_slice(ptr: *const f64, size: i32) -> DoubleArray;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn double_array_drop(value: &mut DoubleArray);

        #[cxx_name = "getRawDataPointer"]
        fn data(self: &DoubleArray) -> *const f64;

        fn size(self: &DoubleArray) -> i32;

        type StringArray = super::StringArray;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn string_array_new() -> StringArray;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn string_array_drop(value: &mut StringArray);

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
