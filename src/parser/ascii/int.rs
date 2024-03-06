use crate::parser::{ascii, byte};
use crate::{error, seq, Either, Parser, ParserResult};

pub fn int<T>(input: &[u8]) -> ParserResult<'_, T, Either<error::AsciiInteger, error::EndOfInput>>
where
    T: Integer,
{
    if input.is_empty() {
        return Err(Either::B(error::EndOfInput));
    }

    let sign = byte(b'-').optional().filter_map(|sign| {
        if !T::IS_SIGNED && sign.is_some() {
            Err(Either::A(error::AsciiInteger))
        } else {
            Ok(sign.is_some())
        }
    });
    let leading_zeroes = byte(b'0').discard_while(|_| true).map(|count| count != 0);
    let digits = ascii::digit.at_most_n_raw(T::MAX_DIGITS);

    sign.map_err(Either::into_a)
        .and_infallible(leading_zeroes)
        .and_infallible(digits)
        .filter_map(|seq!(is_negative, leading_zeroes, digits)| {
            // If the number is zero, all the digits have been stripped away. This needs to be
            // explicitly handled while still erroring on completely empty input.
            if digits.is_empty() {
                return leading_zeroes
                    .then_some(T::ZERO)
                    .ok_or(Either::A(error::AsciiInteger));
            }

            let mut value = T::ZERO;
            for digit in digits {
                let mut digit = (digit - b'0') as i8;
                if T::IS_SIGNED && is_negative {
                    digit *= -1;
                }
                value = value
                    .try_push_digit(digit)
                    .ok_or(Either::A(error::AsciiInteger))?;
            }
            Ok(value)
        })
        .map_err(Either::unify)
        .parse(input)
}

mod sealed {
    pub trait Sealed: Sized {
        const IS_SIGNED: bool;
        const MAX_DIGITS: usize;
        const ZERO: Self;

        fn try_push_digit(self, digit: i8) -> Option<Self>;
    }
}

use self::sealed::Sealed;

pub trait Integer: Sealed {}

macro_rules! impl_integer {
    ($($t:ident $is_signed:literal $max_digits:literal)*) => {$(
        impl Sealed for $t {
            const IS_SIGNED: bool = $is_signed;
            const MAX_DIGITS: usize = $max_digits;
            const ZERO: Self = 0;

            fn try_push_digit(self, digit: i8) -> Option<Self> {
                #[allow(trivial_numeric_casts)]
                self.checked_mul(10)?.checked_add(digit as Self)
            }
        }

        impl Integer for $t {}
    )*};
}

impl_integer! {
    u8 false 3
    i8 true 3

    u16 false 5
    i16 true 5

    u32 false 10
    i32 true 10

    u64 false 20
    i64 true 20

    u128 false 39
    i128 true 39
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParsedItem;

    // Test the basic functionality for both signed and unsigned integers. This includes
    // - ensuring zero (positive and negative) is parsed correctly
    // - ensuring overflow is an error
    // - validating early overflow check
    // - ensuring leading zeroes are stripped
    // - ensuring the parser errors at the end of the input

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_integer_u8() {
        #[cfg_attr(coverage, coverage(off))]
        fn parser(
            input: &[u8],
        ) -> Result<(&[u8], u8), Either<error::AsciiInteger, error::EndOfInput>> {
            int(input).map(ParsedItem::into_parts)
        }

        assert_eq!(parser(b"0"), Ok((b"".as_ref(), 0)));
        assert_eq!(parser(b"1"), Ok((b"".as_ref(), 1)));
        assert_eq!(parser(b"9"), Ok((b"".as_ref(), 9)));
        assert_eq!(parser(b"10"), Ok((b"".as_ref(), 10)));
        assert_eq!(parser(b"00255"), Ok((b"".as_ref(), 255)));
        assert_eq!(parser(b"256"), Err(Either::A(error::AsciiInteger)));
        assert_eq!(parser(b"300"), Err(Either::A(error::AsciiInteger)));
        assert_eq!(parser(b"-0"), Err(Either::A(error::AsciiInteger)));
        assert_eq!(parser(b"-1"), Err(Either::A(error::AsciiInteger)));
        assert_eq!(parser(b"-9"), Err(Either::A(error::AsciiInteger)));
        assert_eq!(parser(b"-10"), Err(Either::A(error::AsciiInteger)));
        assert_eq!(parser(b"-255"), Err(Either::A(error::AsciiInteger)));
        assert_eq!(parser(b"-256"), Err(Either::A(error::AsciiInteger)));
        assert_eq!(parser(b""), Err(Either::B(error::EndOfInput)));
        assert_eq!(parser(b"a"), Err(Either::A(error::AsciiInteger)));
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_integer_i8() {
        #[cfg_attr(coverage, coverage(off))]
        fn parser(
            input: &[u8],
        ) -> Result<(&[u8], i8), Either<error::AsciiInteger, error::EndOfInput>> {
            int(input).map(ParsedItem::into_parts)
        }

        assert_eq!(parser(b"0"), Ok((b"".as_ref(), 0)));
        assert_eq!(parser(b"1"), Ok((b"".as_ref(), 1)));
        assert_eq!(parser(b"9"), Ok((b"".as_ref(), 9)));
        assert_eq!(parser(b"10"), Ok((b"".as_ref(), 10)));
        assert_eq!(parser(b"00127"), Ok((b"".as_ref(), 127)));
        assert_eq!(parser(b"128"), Err(Either::A(error::AsciiInteger)));
        assert_eq!(parser(b"300"), Err(Either::A(error::AsciiInteger)));
        assert_eq!(parser(b"-0"), Ok((b"".as_ref(), 0)));
        assert_eq!(parser(b"-1"), Ok((b"".as_ref(), -1)));
        assert_eq!(parser(b"-9"), Ok((b"".as_ref(), -9)));
        assert_eq!(parser(b"-10"), Ok((b"".as_ref(), -10)));
        assert_eq!(parser(b"-127"), Ok((b"".as_ref(), -127)));
        assert_eq!(parser(b"-128"), Ok((b"".as_ref(), -128)));
        assert_eq!(parser(b"-129"), Err(Either::A(error::AsciiInteger)));
        assert_eq!(parser(b""), Err(Either::B(error::EndOfInput)));
        assert_eq!(parser(b"a"), Err(Either::A(error::AsciiInteger)));
    }

    // With the basics covered, now we can test the bounds of the other integer types. Other checks
    // should not be necessary.

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_integer_u16() {
        assert_eq!(
            int::<u16>(b"65535").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), u16::MAX))
        );
        assert_eq!(int::<u16>(b"65536"), Err(Either::A(error::AsciiInteger)));
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_integer_i16() {
        assert_eq!(
            int::<i16>(b"32767").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), i16::MAX))
        );
        assert_eq!(int::<i16>(b"32768"), Err(Either::A(error::AsciiInteger)));
        assert_eq!(
            int::<i16>(b"-32768").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), i16::MIN))
        );
        assert_eq!(int::<i16>(b"-32769"), Err(Either::A(error::AsciiInteger)));
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_integer_u32() {
        assert_eq!(
            int::<u32>(b"4294967295").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), u32::MAX))
        );
        assert_eq!(
            int::<u32>(b"4294967296"),
            Err(Either::A(error::AsciiInteger))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_integer_i32() {
        assert_eq!(
            int::<i32>(b"2147483647").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), i32::MAX))
        );
        assert_eq!(
            int::<i32>(b"2147483648"),
            Err(Either::A(error::AsciiInteger))
        );
        assert_eq!(
            int::<i32>(b"-2147483648").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), i32::MIN))
        );
        assert_eq!(
            int::<i32>(b"-2147483649"),
            Err(Either::A(error::AsciiInteger))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_integer_u64() {
        assert_eq!(
            int::<u64>(b"18446744073709551615").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), u64::MAX))
        );
        assert_eq!(
            int::<u64>(b"18446744073709551616"),
            Err(Either::A(error::AsciiInteger))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_integer_i64() {
        assert_eq!(
            int::<i64>(b"9223372036854775807").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), i64::MAX))
        );
        assert_eq!(
            int::<i64>(b"9223372036854775808"),
            Err(Either::A(error::AsciiInteger))
        );
        assert_eq!(
            int::<i64>(b"-9223372036854775808").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), i64::MIN))
        );
        assert_eq!(
            int::<i64>(b"-9223372036854775809"),
            Err(Either::A(error::AsciiInteger))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_integer_u128() {
        assert_eq!(
            int::<u128>(b"340282366920938463463374607431768211455").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), u128::MAX))
        );
        assert_eq!(
            int::<u128>(b"340282366920938463463374607431768211456"),
            Err(Either::A(error::AsciiInteger))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_integer_i128() {
        assert_eq!(
            int::<i128>(b"170141183460469231731687303715884105727").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), i128::MAX))
        );
        assert_eq!(
            int::<i128>(b"170141183460469231731687303715884105728"),
            Err(Either::A(error::AsciiInteger))
        );
        assert_eq!(
            int::<i128>(b"-170141183460469231731687303715884105728").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), i128::MIN))
        );
        assert_eq!(
            int::<i128>(b"-170141183460469231731687303715884105729"),
            Err(Either::A(error::AsciiInteger))
        );
    }
}
