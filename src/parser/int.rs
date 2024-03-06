use core::{mem, ptr};

use crate::parser::n_bytes;
use crate::{error, Parser, ParserResult};

pub fn int_be<T>(input: &[u8]) -> ParserResult<'_, T, error::EndOfInput>
where
    T: Integer,
{
    n_bytes(mem::size_of::<T>())
        .map(|bytes| {
            // Safety: `n_bytes` guarantees that exactly `size_of::<T>()` items are read.
            let bytes = unsafe { *(ptr::from_ref(bytes).cast::<T::Array>()) };
            T::from_be_bytes(bytes)
        })
        .parse(input)
}

pub fn int_le<T>(input: &[u8]) -> ParserResult<'_, T, error::EndOfInput>
where
    T: Integer,
{
    n_bytes(mem::size_of::<T>())
        .map(|bytes| {
            // Safety: `n_bytes` guarantees that exactly `size_of::<T>()` items are read.
            let bytes = unsafe { *(ptr::from_ref(bytes).cast::<T::Array>()) };
            T::from_le_bytes(bytes)
        })
        .parse(input)
}

pub fn int_ne<T>(input: &[u8]) -> ParserResult<'_, T, error::EndOfInput>
where
    T: Integer,
{
    n_bytes(mem::size_of::<T>())
        .map(|bytes| {
            // Safety: `n_bytes` guarantees that exactly `size_of::<T>()` items are read.
            let bytes = unsafe { *(ptr::from_ref(bytes).cast::<T::Array>()) };
            T::from_ne_bytes(bytes)
        })
        .parse(input)
}

mod sealed {
    pub trait Sealed {
        type Array: Copy;

        fn from_be_bytes(value: Self::Array) -> Self;
        fn from_le_bytes(value: Self::Array) -> Self;
        fn from_ne_bytes(value: Self::Array) -> Self;
    }
}

use self::sealed::Sealed;

pub trait Integer: Sealed {}

macro_rules! impl_integer {
    ($($t:ident)*) => {$(
        impl Sealed for $t {
            type Array = [u8; mem::size_of::<$t>()];

            fn from_be_bytes(value: Self::Array) -> Self {
                Self::from_be_bytes(value)
            }

            fn from_le_bytes(value: Self::Array) -> Self {
                Self::from_le_bytes(value)
            }

            fn from_ne_bytes(value: Self::Array) -> Self {
                Self::from_ne_bytes(value)
            }
        }

        impl Integer for $t {}
    )*};
}

impl_integer! {
    u8 i8
    u16 i16
    u32 i32
    u64 i64
    u128 i128
}

#[cfg(test)]
mod tests {
    use crate::parser::{int_be, int_le, int_ne};
    use crate::{error, ParsedItem};

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_int_be() {
        let input = [
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc,
            0xde, 0xf0,
        ];

        assert_eq!(
            int_be::<u8>(&input[..1]).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0x12))
        );
        assert_eq!(
            int_be::<u16>(&input[..2]).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0x1234))
        );
        assert_eq!(
            int_be::<u32>(&input[..4]).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0x12345678))
        );
        assert_eq!(
            int_be::<u64>(&input[..8]).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0x123456789abcdef0))
        );
        assert_eq!(
            int_be::<u128>(&input).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0x123456789abcdef0123456789abcdef0))
        );
        assert_eq!(int_be::<u8>(b""), Err(error::EndOfInput));
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_int_le() {
        let input = [
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc,
            0xde, 0xf0,
        ];

        assert_eq!(
            int_le::<u8>(&input[..1]).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0x12))
        );
        assert_eq!(
            int_le::<u16>(&input[..2]).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0x3412))
        );
        assert_eq!(
            int_le::<u32>(&input[..4]).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0x78563412))
        );
        assert_eq!(
            int_le::<u64>(&input[..8]).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0xf0debc9a78563412))
        );
        assert_eq!(
            int_le::<u128>(&input).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0xf0debc9a78563412f0debc9a78563412))
        );
        assert_eq!(int_le::<u8>(b""), Err(error::EndOfInput));
    }

    #[test]
    #[cfg(target_endian = "little")]
    #[cfg_attr(coverage, coverage(off))]
    fn test_int_ne() {
        let input = [
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc,
            0xde, 0xf0,
        ];

        assert_eq!(
            int_ne::<u8>(&input[..1]).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0x12))
        );
        assert_eq!(
            int_ne::<u16>(&input[..2]).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0x3412))
        );
        assert_eq!(
            int_ne::<u32>(&input[..4]).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0x78563412))
        );
        assert_eq!(
            int_ne::<u64>(&input[..8]).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0xf0debc9a78563412))
        );
        assert_eq!(
            int_ne::<u128>(&input).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0xf0debc9a78563412f0debc9a78563412))
        );
        assert_eq!(int_ne::<u8>(b""), Err(error::EndOfInput));
    }

    #[test]
    #[cfg(target_endian = "big")]
    #[cfg_attr(coverage, coverage(off))]
    fn test_int_ne() {
        let input = [
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc,
            0xde, 0xf0,
        ];

        assert_eq!(
            int_ne::<u8>(&input[..1]).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0x12))
        );
        assert_eq!(
            int_ne::<u16>(&input[..2]).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0x1234))
        );
        assert_eq!(
            int_ne::<u32>(&input[..4]).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0x12345678))
        );
        assert_eq!(
            int_ne::<u64>(&input[..8]).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0x123456789abcdef0))
        );
        assert_eq!(
            int_ne::<u128>(&input).map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0x123456789abcdef0123456789abcdef0))
        );
        assert_eq!(int_ne::<u8>(b""), Err(error::EndOfInput));
    }
}
