use crate::define_juce_type;

define_juce_type! {
    JuceString,
    layout = juce::StringLayout,
    cxx_name = "juce::JuceString",
    drop = juce::string_drop,
    default = juce::string_new,
    clone = juce::string_clone,
}

impl JuceString {
    pub fn new(str: impl AsRef<str>) -> Self {
        let str = str.as_ref();
        let data = str.as_ptr().cast();
        str.len()
            .try_into()
            .map(|len| unsafe { Self::from_utf8(data, len) })
            .unwrap_or_default()
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

impl From<&str> for JuceString {
    fn from(value: &str) -> Self {
        JuceString::new(value)
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

impl PartialEq<&str> for JuceString {
    fn eq(&self, other: &&str) -> bool {
        self.as_ref() == *other
    }
}

define_juce_type! {
    CharPointerUTF8,
    layout = juce::CharPointer_UTF8Layout,
    cxx_name = "juce::CharPointer_UTF8",
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum StringLayout {
        Size = 8,
        Alignment = 8,
    }

    enum CharPointer_UTF8Layout {
        Size = 8,
        Alignment = 8,
    }

    unsafe extern "C++" {
        include!("cxx_juce.h");

        type JuceString = crate::juce_core::JuceString;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn string_new() -> JuceString;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn string_drop(self_: &mut JuceString);

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn string_clone(self_: &JuceString) -> JuceString;

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
        assert_eq!(JuceString::default(), JuceString::default());
        assert_eq!(JuceString::new("Hello"), JuceString::new("Hello"));

        assert_ne!(JuceString::new("World"), JuceString::new("Hello"));
    }
}
