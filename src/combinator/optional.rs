use core::convert::Infallible;

use crate::{Combinator, ParsedItem, Parser};

pub const fn optional<'input, P>()
-> impl Combinator<'input, P, Output = Option<P::Output>, Error = Infallible> + Copy
where
    P: Parser<'input>,
{
    move |parser: P| {
        move |input| match parser.parse(input) {
            Ok(parsed_item) => Ok(parsed_item.map_value(Some)),
            Err(_) => Ok(ParsedItem::from_parts(input, None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::any_byte;
    use crate::ParsedItem;

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
