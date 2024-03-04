use crate::parser::any_byte;
use crate::{error, Either, Parser as _, ParserResult};

/// Consume exactly one ASCII letter.
pub fn alphabetic(
    input: &[u8],
) -> ParserResult<'_, char, Either<error::AsciiAlphabetic, error::EndOfInput>> {
    any_byte
        .filter_map(|b| {
            b.is_ascii_alphabetic()
                .then_some(b as char)
                .ok_or(error::AsciiAlphabetic)
        })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParsedItem;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_alphabetic_valid() {
        assert_eq!(
            alphabetic(b"abc").map(ParsedItem::into_parts),
            Ok((b"bc".as_ref(), 'a'))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_alphabetic_error_invalid() {
        assert_eq!(alphabetic(b"123"), Err(Either::A(error::AsciiAlphabetic)));
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_alphabetic_error_end_of_input() {
        assert_eq!(alphabetic(b""), Err(Either::B(error::EndOfInput)));
    }
}
