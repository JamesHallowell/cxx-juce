use crate::{define_array_type, define_juce_type, juce_core::JuceString};
use std::iter::FromIterator;

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
    IntArrayIter,
    IntArrayIterRef,
    data = IntArray::data,
    from_slice = juce::int_array_new_from_slice,
    add_ref = IntArray::add
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
    FloatArrayIter,
    FloatArrayIterRef,
    data = FloatArray::data,
    from_slice = juce::float_array_new_from_slice,
    add_ref = FloatArray::add
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
    DoubleArrayIter,
    DoubleArrayIterRef,
    data = DoubleArray::data,
    from_slice = juce::double_array_new_from_slice,
    add_ref = DoubleArray::add
}

define_juce_type! {
    StringArray,
    layout = juce::StringArrayLayout,
    cxx_name = "juce::StringArray",
    drop = juce::string_array_drop,
    default = juce::string_array_new,
    clone = juce::string_array_clone,
    equality = juce::string_array_equality
}

define_array_type! {
    StringArray,
    JuceString,
    StringArrayIter,
    StringArrayIterRef,
    data = StringArray::data,
    add = StringArray::add,
}

pub(crate) use juce::ArrayLayout;

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum ArrayLayout {
        Size = 16,
        Alignment = 8,
    }

    enum StringArrayLayout {
        Size = 16,
        Alignment = 8,
    }

    unsafe extern "C++" {
        include!("cxx_juce.h");

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

        #[cxx_name = "size"]
        fn len(self: &IntArray) -> i32;

        fn add(self: &mut IntArray, value: &i32);

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

        #[cxx_name = "size"]
        fn len(self: &FloatArray) -> i32;

        fn add(self: &mut FloatArray, value: &f32);

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

        #[cxx_name = "size"]
        fn len(self: &DoubleArray) -> i32;

        fn add(self: &mut DoubleArray, value: &f64);

        type StringArray = super::StringArray;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn string_array_new() -> StringArray;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn string_array_drop(value: &mut StringArray);

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn string_array_clone(value: &StringArray) -> StringArray;

        #[namespace = "cxx_juce"]
        #[cxx_name = "eq"]
        fn string_array_equality(value: &StringArray, other: &StringArray) -> bool;

        #[namespace = "cxx_juce"]
        #[cxx_name = "begin"]
        fn data(self: &StringArray) -> *const JuceString;

        #[cxx_name = "add"]
        fn add(self: &mut StringArray, value: JuceString);

        #[cxx_name = "getReference"]
        fn get_reference(self: &StringArray, index: i32) -> &JuceString;

        #[cxx_name = "size"]
        fn len(self: &StringArray) -> i32;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::juce_core::JuceString;

    #[test]
    fn string_array_iter() {
        let mut array = StringArray::default();
        array.add("Hello".into());
        array.add("World".into());
        array.add("!".into());

        assert_eq!(array.len(), 3);

        let mut iter = array.into_iter();
        assert_eq!(iter.next(), Some(JuceString::new("Hello")));
        assert_eq!(iter.next(), Some(JuceString::new("World")));
        assert_eq!(iter.next(), Some(JuceString::new("!")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn array_from_slice() {
        let array = [1, 2, 3];
        let array = juce::IntArray::from(array.as_slice());

        assert_eq!(array.len(), 3);
        assert_eq!(array.as_ref(), &[1, 2, 3]);
    }

    #[test]
    fn string_array_as_slice() {
        let array: StringArray = ["Hello", "World", "!"].into_iter().collect();

        assert_eq!(array.as_ref(), ["Hello", "World", "!"]);
    }

    #[test]
    fn get_at_index() {
        let array: IntArray = [1, 2, 3].into_iter().collect();

        assert_eq!(array.get(0), Some(&1));
        assert_eq!(array.get(1), Some(&2));
        assert_eq!(array.get(2), Some(&3));
        assert_eq!(array.get(3), None);
    }

    #[test]
    fn get_ref_at_index() {
        let array: StringArray = ["A", "B", "C"].into_iter().collect();

        assert_eq!(array.get(0), Some(&JuceString::from("A")));
        assert_eq!(array.get(1), Some(&JuceString::from("B")));
        assert_eq!(array.get(2), Some(&JuceString::from("C")));
        assert_eq!(array.get(3), None);
    }
}
