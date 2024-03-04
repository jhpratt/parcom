use crate::{Combinator, Either, Parser};

#[rustfmt::skip] // rust-lang/rustfmt#3599
pub const fn or<'input, P1, P2>(
    p2: P2,
) -> impl Combinator<
    'input,
    P1,
    Output = Either<P1::Output, P2::Output>,
    Error = (P1::Error, P2::Error),
> + Copy
where
    P1: Parser<'input>,
    P2: Parser<'input>,
{
    move |p1: P1| {
        move |input| match p1.parse(input) {
            Ok(parsed_item) => Ok(parsed_item.map_value(Either::A)),
            Err(err1) => match p2.parse(input) {
                Ok(parsed_item) => Ok(parsed_item.map_value(Either::B)),
                Err(err2) => Err((err1, err2)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{byte, verbatim};
    use crate::{error, ParsedItem};

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_or_valid() {
        assert_eq!(
            byte(b'a')
                .or(verbatim(b"bc"))
                .parse(b"a")
                .map(ParsedItem::into_parts),
            Ok((b"".as_ref(), Either::A(b'a')))
        );
        assert_eq!(
            byte(b'a')
                .or(verbatim(b"bc"))
                .parse(b"bc")
                .map(ParsedItem::into_parts),
            Ok((b"".as_ref(), Either::B(b"bc".as_ref())))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_or_invalid() {
        assert_eq!(
            byte(b'a')
                .or(verbatim(b"bc"))
                .parse(b"b")
                .map(ParsedItem::into_parts),
            Err((Either::A(error::Byte), Either::B(error::EndOfInput)))
        );
    }
}
