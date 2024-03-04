use crate::parser::any_byte;
use crate::{error, Either, Parser as _, ParserResult};

/// Consume exactly one ASCII digit.
pub fn digit(input: &[u8]) -> ParserResult<'_, u8, Either<error::AsciiDigit, error::EndOfInput>> {
    any_byte
        .filter_map(|b| {
            b.is_ascii_digit()
                .then_some(b - b'0')
                .ok_or(error::AsciiDigit)
        })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParsedItem;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_digit_valid() {
        assert_eq!(
            digit(b"0").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0))
        );
        assert_eq!(
            digit(b"123").map(ParsedItem::into_parts),
            Ok((b"23".as_ref(), 1))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_digit_invalid() {
        assert_eq!(
            digit(b"a").map(ParsedItem::into_parts),
            Err(Either::A(error::AsciiDigit))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_digit_end_of_input() {
        assert_eq!(digit(b""), Err(Either::B(error::EndOfInput)));
    }
}
