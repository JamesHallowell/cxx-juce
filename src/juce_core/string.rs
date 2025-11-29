use crate::{define_juce_type, static_assert_size_and_alignment};

define_juce_type!(
    JuceString,
    size = 8,
    align = 8,
    check_with = juce::StringLayout
);

unsafe impl cxx::ExternType for JuceString {
    type Id = cxx::type_id!("juce::String");
    type Kind = cxx::kind::Trivial;
}

impl Default for JuceString {
    fn default() -> Self {
        Self::empty()
    }
}

impl JuceString {
    pub fn empty() -> JuceString {
        juce::construct_string()
    }

    pub fn new(str: impl AsRef<str>) -> Self {
        let str = str.as_ref();
        let data = str.as_ptr().cast();
        str.len()
            .try_into()
            .map(|len| unsafe { Self::from_utf8(data, len) })
            .unwrap_or_default()
    }
}

impl Drop for JuceString {
    fn drop(&mut self) {
        juce::drop_string(self);
    }
}

impl AsRef<str> for JuceString {
    fn as_ref(&self) -> &str {
        let data = self.to_utf8().get_address().cast();
        let len = self.get_num_bytes_as_utf8();

        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(data, len)) }
    }
}

impl From<JuceString> for String {
    fn from(value: JuceString) -> Self {
        value.as_ref().to_string()
    }
}

impl std::fmt::Display for JuceString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl std::fmt::Debug for JuceString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_ref())
    }
}

impl PartialEq for JuceString {
    fn eq(&self, other: &Self) -> bool {
        juce::eq_string(self, other)
    }
}

impl PartialEq<str> for JuceString {
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}

define_juce_type!(
    CharPointerUTF8,
    size = 8,
    align = 8,
    check_with = juce::CharPointerUTF8Layout
);

unsafe impl cxx::ExternType for CharPointerUTF8 {
    type Id = cxx::type_id!("juce::CharPointer_UTF8");
    type Kind = cxx::kind::Trivial;
}
static_assert_size_and_alignment!(CharPointerUTF8, juce::CharPointerUTF8Layout);

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum StringLayout {
        Size = 8,
        Alignment = 8,
    }

    enum CharPointerUTF8Layout {
        Size = 8,
        Alignment = 8,
    }

    unsafe extern "C++" {
        include!("cxx_juce.h");
        include!("cxx_juce_core/cxx_juce_core.h");

        #[cxx_name = "String"]
        type JuceString = super::JuceString;

        #[namespace = "cxx_juce"]
        #[rust_name = "construct_string"]
        fn construct() -> JuceString;

        #[namespace = "cxx_juce"]
        #[rust_name = "drop_string"]
        fn drop(value: &mut JuceString);

        #[cxx_name = "fromUTF8"]
        #[Self = "JuceString"]
        unsafe fn from_utf8(data: *const c_char, len: i32) -> JuceString;

        #[cxx_name = "isEmpty"]
        fn is_empty(self: &JuceString) -> bool;

        #[namespace = "cxx_juce"]
        #[rust_name = "eq_string"]
        fn eq(a: &JuceString, b: &JuceString) -> bool;

        #[cxx_name = "toUTF8"]
        fn to_utf8(self: &JuceString) -> CharPointerUTF8;

        #[cxx_name = "getNumBytesAsUTF8"]
        fn get_num_bytes_as_utf8(self: &JuceString) -> usize;

        #[cxx_name = "CharPointer_UTF8"]
        type CharPointerUTF8 = super::CharPointerUTF8;

        #[cxx_name = "getAddress"]
        fn get_address(self: &CharPointerUTF8) -> *mut c_char;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_a_juce_string() {
        let string = JuceString::new("Hello JUCE 🧃");
        assert_eq!(string.as_ref(), "Hello JUCE 🧃");
    }

    #[test]
    fn compare_strings() {
        assert_eq!(JuceString::empty(), JuceString::empty());
        assert_eq!(JuceString::new("Hello"), JuceString::new("Hello"));

        assert_ne!(JuceString::new("World"), JuceString::new("Hello"));
    }
}
