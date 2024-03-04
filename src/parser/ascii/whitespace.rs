use crate::parser::any_byte;
use crate::{error, Either, Parser as _, ParserResult};

/// Consume exactly one ASCII whitespace character.
pub fn whitespace(
    input: &[u8],
) -> ParserResult<'_, char, Either<error::AsciiWhitespace, error::EndOfInput>> {
    any_byte
        .filter_map(|b| {
            b.is_ascii_whitespace()
                .then_some(b as char)
                .ok_or(error::AsciiWhitespace)
        })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParsedItem;

    #[test]
    fn test_whitespace_valid() {
        assert_eq!(
            whitespace(b" ").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), ' '))
        );
        assert_eq!(
            whitespace(b"\t").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), '\t'))
        );
        assert_eq!(
            whitespace(b"\n").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), '\n'))
        );
        assert_eq!(
            whitespace(b"\r").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), '\r'))
        );
        assert_eq!(
            whitespace(b"\x0C").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), '\x0C'))
        );
    }

    #[test]
    fn test_whitespace_invalid() {
        assert_eq!(whitespace(b"1"), Err(Either::A(error::AsciiWhitespace)));
    }
}
