use crate::{Combinator, Parser};

pub const fn exactly_n_raw<'input, P>(
    n: usize,
) -> impl Combinator<'input, P, Output = &'input [u8], Error = P::Error> + Copy
where
    P: Parser<'input>,
{
    move |parser: P| parser.n_to_m_raw(n, n)
}

#[cfg(test)]
mod tests {
    use crate::parser::ascii;
    use crate::{error, Either, ParsedItem, Parser as _};

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_exactly_n_raw_success() {
        let parser = ascii::alphabetic.exactly_n_raw(2);

        assert_eq!(
            parser.parse(b"ab").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b"ab".as_ref()))
        );
        assert_eq!(
            parser.parse(b"abc").map(ParsedItem::into_parts),
            Ok((b"c".as_ref(), b"ab".as_ref()))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_exactly_n_raw_error() {
        let parser = ascii::alphabetic.exactly_n_raw(2);

        assert_eq!(parser.parse(b""), Err(Either::B(error::EndOfInput)));
        assert_eq!(parser.parse(b"a"), Err(Either::B(error::EndOfInput)));
    }
}
