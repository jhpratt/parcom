use core::convert::Infallible;

use crate::{Combinator, ParsedItem, Parser};

pub const fn and_infallible<'input, P1, P2>(
    p2: P2,
) -> impl Combinator<'input, P1, Output = (P1::Output, P2::Output), Error = P1::Error> + Copy
where
    P1: Parser<'input>,
    P2: Parser<'input, Error = Infallible>,
{
    move |p1: P1| {
        move |input| {
            let (input, p1_value) = p1.parse(input)?.into_parts();
            let (input, p2_value) = match p2.parse(input) {
                Ok(parsed) => parsed.into_parts(),
                Err(err) => match err {},
            };
            Ok(ParsedItem::from_parts(input, (p1_value, p2_value)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{any_byte, success};
    use crate::{error, ParsedItem, Parser as _};

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_and_infallible_success() {
        let parser = any_byte.and_infallible(success(()));
        assert_eq!(
            parser.parse(b"abc").map(ParsedItem::into_parts),
            Ok((b"bc".as_ref(), (b'a', ())))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_and_infallible_error() {
        let parser = any_byte.and_infallible(success(()));
        assert_eq!(parser.parse(b""), Err(error::EndOfInput));
    }
}
