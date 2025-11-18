use crate::define_juce_type;

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum BigIntegerLayout {
        Size = 40,
        Alignment = 8,
    }

    unsafe extern "C++" {
        include!("cxx_juce.h");
        include!("cxx_juce_core/cxx_juce_core.h");

        type BigInteger = super::BigInteger;

        #[namespace = "cxx_juce"]
        #[rust_name = "construct_big_integer"]
        fn construct() -> BigInteger;

        #[namespace = "cxx_juce"]
        #[rust_name = "drop_big_integer"]
        fn drop(value: &mut BigInteger);

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

define_juce_type!(
    BigInteger,
    size = 40,
    align = 8,
    check_with = juce::BigIntegerLayout
);

unsafe impl cxx::ExternType for BigInteger {
    type Id = cxx::type_id!("juce::BigInteger");
    type Kind = cxx::kind::Trivial;
}

impl Default for BigInteger {
    fn default() -> Self {
        juce::construct_big_integer()
    }
}

impl Drop for BigInteger {
    fn drop(&mut self) {
        juce::drop_big_integer(self);
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
