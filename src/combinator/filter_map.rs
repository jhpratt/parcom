use crate::{Combinator, Either, Parser};

pub const fn filter_map<'input, P, Output, Error, F>(
    f: F,
) -> impl Combinator<'input, P, Output = Output, Error = Either<Error, P::Error>> + Copy
where
    P: Parser<'input>,
    F: Fn(P::Output) -> Result<Output, Error> + Copy,
{
    move |parser: P| {
        move |input| match parser.parse(input) {
            Ok(parsed_item) => parsed_item.filter_map_value(f).map_err(Either::A),
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
                .filter_map(
                    #[cfg_attr(coverage, coverage(off))]
                    |b| if b == b'a' { Err(()) } else { Ok(b) }
                )
                .parse(b"b")
                .map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b'b'))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_filter_map_filtered() {
        assert_eq!(
            any_byte
                .filter_map(
                    #[cfg_attr(coverage, coverage(off))]
                    |b| if b == b'a' { Err(()) } else { Ok(b) }
                )
                .parse(b"a"),
            Err(Either::A(()))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_filter_map_error() {
        assert_eq!(
            n_bytes(2).filter_map(Ok).parse(b"a"),
            Err(Either::<(), _>::B(error::EndOfInput))
        );
    }
}
