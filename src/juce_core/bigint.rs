use crate::define_juce_type;

define_juce_type! {
    BigInteger,
    layout = juce::BigIntegerLayout,
    cxx_name = "juce::BigInteger",
    drop = juce::big_integer_drop,
    default = juce::big_integer_new,
}

impl std::fmt::Debug for BigInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string(16, 2))
    }
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum BigIntegerLayout {
        Size = 40,
        Alignment = 8,
    }

    unsafe extern "C++" {
        include!("cxx_juce.h");

        type JuceString = crate::juce_core::JuceString;
        type BigInteger = super::BigInteger;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn big_integer_new() -> BigInteger;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn big_integer_drop(self_: &mut BigInteger);

        #[cxx_name = "countNumberOfSetBits"]
        fn count_number_of_set_bits(self: &BigInteger) -> i32;

        fn clear(self: &mut BigInteger) -> &mut BigInteger;

        #[cxx_name = "setRange"]
        fn set_range(
            self: &mut BigInteger,
            start_bit: i32,
            num_bits: i32,
            should_be_set: bool,
        ) -> &mut BigInteger;

        #[cxx_name = "toString"]
        fn to_string(self: &BigInteger, base: i32, min_chars: i32) -> JuceString;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_bits_in_big_integer() {
        let mut integer = BigInteger::default();
        assert_eq!(integer.count_number_of_set_bits(), 0);

        integer.set_range(0, 8, true);
        assert_eq!(integer.count_number_of_set_bits(), 8);

        integer.clear().set_range(0, 3, true);
        assert_eq!(integer.count_number_of_set_bits(), 3);
    }

    #[test]
    fn to_string() {
        let mut integer = BigInteger::default();

        assert_eq!(integer.to_string(10, 1), "0");

        integer.set_range(0, 2, true);

        assert_eq!(integer.to_string(10, 1), "3");
        assert_eq!(integer.to_string(16, 2), "03");

        integer.set_range(0, 4, true);

        assert_eq!(integer.to_string(10, 1), "15");
        assert_eq!(integer.to_string(16, 1), "f");
    }

    #[test]
    fn debug() {
        let mut integer = BigInteger::default();
        assert_eq!(format!("{integer:?}"), "00");

        integer.set_range(0, 3, true);
        assert_eq!(format!("{integer:?}"), "07");

        integer.set_range(0, 8, true);
        assert_eq!(format!("{integer:?}"), "ff");

        integer.set_range(0, 12, true);
        assert_eq!(format!("{integer:?}"), "fff");
    }
}
