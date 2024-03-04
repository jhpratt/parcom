use crate::{error, hrtb_hack, Either, ParsedItem, Parser};

pub fn utf8_char(
    c: char,
) -> impl for<'input> Parser<'input, Output = char, Error = Either<error::Utf8Char, error::EndOfInput>> {
    hrtb_hack(move |input: &[u8]| {
        if input.len() < c.len_utf8() {
            return Err(Either::B(error::EndOfInput));
        }

        let mut buf = [0; 4];
        let bytes = c.encode_utf8(&mut buf).as_bytes();

        match input.strip_prefix(bytes) {
            Some(remaining_input) => Ok(ParsedItem::from_parts(remaining_input, c)),
            None => Err(Either::A(error::Utf8Char)),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_utf8_char_valid() {
        // 1 byte
        assert_eq!(
            utf8_char('$').parse(b"$0").map(ParsedItem::into_parts),
            Ok((b"0".as_ref(), '$'))
        );
        // 2 bytes
        assert_eq!(
            utf8_char('£')
                .parse("£0".as_bytes())
                .map(ParsedItem::into_parts),
            Ok((b"0".as_ref(), '£'))
        );
        // 3 bytes
        assert_eq!(
            utf8_char('€')
                .parse("€0".as_bytes())
                .map(ParsedItem::into_parts),
            Ok((b"0".as_ref(), '€'))
        );
        // 4 bytes
        assert_eq!(
            utf8_char('🦀')
                .parse("🦀0".as_bytes())
                .map(ParsedItem::into_parts),
            Ok((b"0".as_ref(), '🦀'))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_utf8_char_invalid() {
        assert_eq!(utf8_char('a').parse(b"bcd"), Err(Either::A(error::Utf8Char)));
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_utf8_char_end_of_input() {
        assert_eq!(utf8_char('$').parse(b""), Err(Either::B(error::EndOfInput)));
        assert_eq!(utf8_char('£').parse(b"a"), Err(Either::B(error::EndOfInput)));
        assert_eq!(utf8_char('€').parse(b"ab"), Err(Either::B(error::EndOfInput)));
        assert_eq!(utf8_char('🦀').parse(b"abc"), Err(Either::B(error::EndOfInput)));
    }
}
