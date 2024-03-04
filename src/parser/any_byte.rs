use crate::{error, ParsedItem, ParserResult};

/// Consume exactly one byte.
pub const fn any_byte(input: &[u8]) -> ParserResult<'_, u8, error::EndOfInput> {
    match input {
        [c, remaining @ ..] => Ok(ParsedItem::from_parts(remaining, *c)),
        _ => Err(error::EndOfInput),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_any_byte_success() {
        assert_eq!(
            any_byte(b"abc").map(ParsedItem::into_parts),
            Ok((b"bc".as_ref(), b'a'))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_any_byte_end_of_input() {
        assert_eq!(any_byte(b""), Err(error::EndOfInput));
    }
}
