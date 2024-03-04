use crate::{Combinator, Parser};

pub const fn map_err<'input, F, NewError, P>(
    f: F,
) -> impl Combinator<'input, P, Output = P::Output, Error = NewError> + Copy
where
    F: Fn(P::Error) -> NewError + Copy,
    P: Parser<'input>,
{
    move |parser: P| move |input| parser.parse(input).map_err(f)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{any_byte, n_bytes};
    use crate::ParsedItem;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_map_err_success() {
        assert_eq!(
            any_byte
                .map_err(Some)
                .parse(b"a")
                .map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b'a'))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_map_err_error() {
        assert_eq!(
            n_bytes(2)
                .map_err(
                    #[cfg_attr(coverage, coverage(off))]
                    |_| ()
                )
                .parse(b"a"),
            Err(())
        );
    }
}
