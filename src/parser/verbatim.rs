use crate::{error, hrtb_hack, Either, ParsedItem, Parser};

pub fn verbatim(
    expected: &[u8],
) -> impl for<'input> Parser<
    'input,
    Output = &[u8],
    Error = Either<error::NonMatchingInput, error::EndOfInput>,
> {
    hrtb_hack(move |input| {
        if input.len() < expected.len() {
            return Err(Either::B(error::EndOfInput));
        }

        match input.strip_prefix(expected) {
            Some(remaining_input) => Ok(ParsedItem::from_parts(remaining_input, expected)),
            None => Err(Either::A(error::NonMatchingInput)),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_verbatim_valid() {
        assert_eq!(
            verbatim(b"abc")
                .parse(b"abcdef")
                .map(ParsedItem::into_parts),
            Ok((b"def".as_ref(), b"abc".as_ref()))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_verbatim_invalid() {
        assert_eq!(
            verbatim(b"abc").parse(b"def"),
            Err(Either::A(error::NonMatchingInput))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_verbatim_end_of_input() {
        assert_eq!(
            verbatim(b"abc").parse(b"ab"),
            Err(Either::B(error::EndOfInput))
        );
    }
}
