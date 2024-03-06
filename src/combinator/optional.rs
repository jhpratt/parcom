use core::convert::Infallible;

use crate::{ParsedItem, Parser};

pub const fn optional<'input, P>(
    parser: P,
) -> impl Parser<'input, Output = Option<P::Output>, Error = Infallible>
where
    P: Parser<'input>,
{
    move |input| match parser.parse(input) {
        Ok(parsed_item) => Ok(parsed_item.map_value(Some)),
        Err(_) => Ok(ParsedItem::from_parts(input, None)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::any_byte;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_optional_valid() {
        assert_eq!(
            any_byte.optional().parse(b"a").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), Some(b'a')))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_optional_invalid() {
        assert_eq!(
            any_byte.optional().parse(b"").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), None))
        );
    }
}
