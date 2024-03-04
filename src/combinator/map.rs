use crate::{Combinator, Parser};

pub const fn map<'input, F, NewOutput, P>(
    f: F,
) -> impl Combinator<'input, P, Output = NewOutput, Error = P::Error> + Copy
where
    F: Fn(P::Output) -> NewOutput + Copy,
    P: Parser<'input>,
{
    move |parser: P| move |input| parser.parse(input).map(|parsed| parsed.map_value(f))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{any_byte, n_bytes};
    use crate::{error, ParsedItem};

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_map_success() {
        assert_eq!(
            any_byte
                .map(
                    #[cfg_attr(coverage, coverage(off))]
                    |b| b + 1
                )
                .parse(b"a")
                .map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b'b'))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_map_error() {
        assert_eq!(n_bytes(2).map(Some).parse(b"a"), Err(error::EndOfInput));
    }
}
