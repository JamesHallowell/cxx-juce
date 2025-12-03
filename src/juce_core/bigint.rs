use crate::define_juce_type;

define_juce_type! {
    BigInteger,
    layout = juce::BigIntegerLayout,
    cxx_name = "juce::BigInteger",
    drop = juce::big_integer_drop,
    default = juce::big_integer_new,
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum BigIntegerLayout {
        Size = 40,
        Alignment = 8,
    }

    unsafe extern "C++" {
        include!("cxx_juce.h");

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
}
