use crate::Parser;

pub fn discard<'input, P>(parser: P) -> impl Parser<'input, Output = (), Error = P::Error>
where
    P: Parser<'input>,
{
    parser.map(|_| ())
}

#[cfg(test)]
mod tests {
    use crate::parser::any_byte;
    use crate::{error, ParsedItem, Parser as _};

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_discard_success() {
        let parser = any_byte.discard();

        assert_eq!(
            parser.parse(b"a").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), ()))
        );
        assert_eq!(
            parser.parse(b"ab").map(ParsedItem::into_parts),
            Ok((b"b".as_ref(), ()))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_discard_error() {
        let parser = any_byte.discard();

        assert_eq!(parser.parse(b""), Err(error::EndOfInput));
    }
}
