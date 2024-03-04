use crate::parser::any_byte;
use crate::{error, Either, Parser as _, ParserResult};

/// Consume exactly one ASCII hex digit.
pub fn hex_digit(
    input: &[u8],
) -> ParserResult<'_, u8, Either<error::AsciiHexDigit, error::EndOfInput>> {
    any_byte
        .filter_map(|c| match c {
            b'0'..=b'9' => Ok(c - b'0'),
            b'a'..=b'f' => Ok(c - b'a' + 10),
            b'A'..=b'F' => Ok(c - b'A' + 10),
            _ => Err(error::AsciiHexDigit),
        })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParsedItem;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_hex_digit_valid() {
        assert_eq!(
            hex_digit(b"1").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 1))
        );
        assert_eq!(
            hex_digit(b"a").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 10))
        );
        assert_eq!(
            hex_digit(b"F").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 15))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_hex_digit_invalid() {
        assert_eq!(hex_digit(b"g"), Err(Either::A(error::AsciiHexDigit)));
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_hex_digit_end_of_input() {
        assert_eq!(hex_digit(b""), Err(Either::B(error::EndOfInput)));
    }
}
