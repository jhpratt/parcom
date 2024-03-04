use crate::parser::any_byte;
use crate::{error, hrtb_hack, Either, Parser};

/// Consume the exact byte.
pub fn byte(
    expected: u8,
) -> impl for<'input> Parser<'input, Output = u8, Error = Either<error::Byte, error::EndOfInput>> {
    hrtb_hack(move |input| {
        any_byte
            .filter_map(|b| (b == expected).then_some(b).ok_or(error::Byte))
            .parse(input)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParsedItem;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_byte_valid() {
        assert_eq!(
            byte(b'a').parse(b"a").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b'a'))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_byte_invalid() {
        assert_eq!(byte(b'a').parse(b"b"), Err(Either::A(error::Byte)));
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_byte_end_of_input() {
        assert_eq!(byte(b'a').parse(b""), Err(Either::B(error::EndOfInput)));
    }
}
