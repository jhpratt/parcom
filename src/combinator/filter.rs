use crate::{error, Combinator, Either, Parser};

pub const fn filter<'input, P, F>(
    f: F,
) -> impl Combinator<'input, P, Output = P::Output, Error = Either<error::Filter, P::Error>> + Copy
where
    P: Parser<'input>,
    F: Fn(&P::Output) -> bool + Copy,
{
    move |parser: P| {
        move |input| match parser.parse(input) {
            Ok(parsed_item) if f(parsed_item.value()) => Ok(parsed_item),
            Ok(_) => Err(Either::A(error::Filter)),
            Err(err) => Err(Either::B(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{any_byte, n_bytes};
    use crate::{error, ParsedItem, Parser};

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_filter_map_success() {
        assert_eq!(
            any_byte
                .filter(
                    #[cfg_attr(coverage, coverage(off))]
                    |&b| b == b'a'
                )
                .parse(b"a")
                .map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b'a'))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_filter_map_filtered() {
        assert_eq!(
            any_byte
                .filter(
                    #[cfg_attr(coverage, coverage(off))]
                    |&b| b == b'a'
                )
                .parse(b"b"),
            Err(Either::A(error::Filter))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_filter_map_error() {
        assert_eq!(
            n_bytes(2)
                .filter(
                    #[cfg_attr(coverage, coverage(off))]
                    |&b| b == b"ab"
                )
                .parse(b"a"),
            Err(Either::B(error::EndOfInput))
        );
    }
}
