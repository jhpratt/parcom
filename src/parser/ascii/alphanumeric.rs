use crate::parser::any_byte;
use crate::{error, Either, Parser as _, ParserResult};

/// Consume exactly one ASCII letter or digit.
pub fn alphanumeric(
    input: &[u8],
) -> ParserResult<'_, char, Either<error::AsciiAlphanumeric, error::EndOfInput>> {
    any_byte
        .filter_map(|b| {
            b.is_ascii_alphanumeric()
                .then_some(b as char)
                .ok_or(error::AsciiAlphanumeric)
        })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParsedItem;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_alphanumeric_valid() {
        assert_eq!(
            alphanumeric(b"abc").map(ParsedItem::into_parts),
            Ok((b"bc".as_ref(), 'a'))
        );
        assert_eq!(
            alphanumeric(b"123").map(ParsedItem::into_parts),
            Ok((b"23".as_ref(), '1'))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_alphanumeric_invalid() {
        assert_eq!(
            alphanumeric(b"/abc").map(ParsedItem::into_parts),
            Err(Either::A(error::AsciiAlphanumeric))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_alphanumeric_end_of_input() {
        assert_eq!(alphanumeric(b""), Err(Either::B(error::EndOfInput)));
    }
}
