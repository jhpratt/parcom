use crate::{Combinator, Either, ParsedItem, Parser};

#[rustfmt::skip] // rust-lang/rustfmt#3599
pub const fn and<'input, P1, P2>(
    p2: P2,
) -> impl Combinator<
    'input,
    P1,
    Output = (P1::Output, P2::Output),
    Error = Either<P1::Error, P2::Error>
> + Copy
where
    P1: Parser<'input>,
    P2: Parser<'input>,
{
    move |p1: P1| {
        move |input| {
            let (input, p1_value) = match p1.parse(input) {
                Ok(parsed) => parsed.into_parts(),
                Err(err) => return Err(Either::A(err)),
            };
            let (input, p2_value) = match p2.parse(input) {
                Ok(parsed) => parsed.into_parts(),
                Err(err) => return Err(Either::B(err)),
            };
            Ok(ParsedItem::from_parts(
                input,
                (p1_value, p2_value),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::any_byte;
    use crate::{error, Either, ParsedItem, Parser as _};

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_and_success() {
        let parser = any_byte.and(any_byte);

        assert_eq!(
            parser.parse(b"abc").map(ParsedItem::into_parts),
            Ok((b"c".as_ref(), (b'a', b'b')))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_and_error() {
        let parser = any_byte.and(any_byte);

        assert_eq!(parser.parse(b""), Err(Either::A(error::EndOfInput)));
        assert_eq!(parser.parse(b"a"), Err(Either::B(error::EndOfInput)));
    }
}
