use alloc::vec::Vec;

use crate::{Combinator, Parser};

pub const fn exactly_n<'input, P>(
    n: usize,
) -> impl Combinator<'input, P, Output = Vec<P::Output>, Error = P::Error> + Copy
where
    P: Parser<'input>,
{
    move |parser: P| parser.n_to_m(n, n)
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use crate::parser::ascii;
    use crate::{error, Either, ParsedItem, Parser as _};

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_exactly_n_success() {
        let parser = ascii::alphabetic.exactly_n(2);

        assert_eq!(
            parser.parse(b"ab").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), vec!['a', 'b']))
        );
        assert_eq!(
            parser.parse(b"abc").map(ParsedItem::into_parts),
            Ok((b"c".as_ref(), vec!['a', 'b']))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_exactly_n_error() {
        let parser = ascii::alphabetic.exactly_n(2);

        assert_eq!(parser.parse(b""), Err(Either::B(error::EndOfInput)));
        assert_eq!(parser.parse(b"a"), Err(Either::B(error::EndOfInput)));
    }
}
