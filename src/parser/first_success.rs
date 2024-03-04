use crate::Parser;

pub fn first_success<'input, const N: usize, P>(
    parsers: [P; N],
) -> impl Parser<'input, Output = P::Output, Error = P::Error>
where
    P: Parser<'input>,
{
    // Prevent compilation if the array is empty. Equivalent to `const { assert!(N > 0); }`.
    // Because this is the only way to construct `FirstSuccess`, it is guaranteed that any
    // instance of the type will contain at least one element.
    struct SizeAssertion<const N: usize>;
    impl<const N: usize> SizeAssertion<N> {
        const IS_VALID: () = assert!(N > 0);
    }
    #[allow(path_statements, clippy::no_effect)]
    {
        SizeAssertion::<N>::IS_VALID;
    }

    move |input| {
        // Safety: `FirstSuccess` is guaranteed to contain at least one parser.
        let (first_parser, remaining_parsers) = unsafe { parsers.split_first().unwrap_unchecked() };

        let first_error = match first_parser.parse(input) {
            Ok(parsed_item) => return Ok(parsed_item),
            Err(err) => err,
        };

        for parser in remaining_parsers {
            if let Ok(parsed_item) = parser.parse(input) {
                return Ok(parsed_item);
            }
        }

        Err(first_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::verbatim;
    use crate::{error, Either, ParsedItem};

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_first_success_valid() {
        let parser = first_success([verbatim(b"a"), verbatim(b"b"), verbatim(b"ab")]);
        assert_eq!(
            parser.parse(b"a").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b"a".as_ref()))
        );
        assert_eq!(
            parser.parse(b"b").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b"b".as_ref()))
        );
        assert_eq!(
            parser.parse(b"ab").map(ParsedItem::into_parts),
            Ok((b"b".as_ref(), b"a".as_ref()))
        );
    }

    #[test]
    fn test_first_success_invalid() {
        let parser = first_success([verbatim(b"a"), verbatim(b"b"), verbatim(b"ab")]);
        assert_eq!(parser.parse(b"c"), Err(Either::A(error::NonMatchingInput)));
        assert_eq!(parser.parse(&[]), Err(Either::B(error::EndOfInput)));
    }
}
