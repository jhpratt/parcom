use crate::{error, ParsedItem, Parser};

pub fn n_bytes(
    n: usize,
) -> impl for<'input> Parser<'input, Output = &'input [u8], Error = error::EndOfInput> {
    const fn hrtb_hack<Error, F>(f: F) -> F
    where
        F: Fn(&[u8]) -> crate::ParserResult<'_, &[u8], Error>,
    {
        f
    }

    hrtb_hack(move |input| {
        if input.len() < n {
            return Err(error::EndOfInput);
        }

        let (value, input) = input.split_at(n);
        Ok(ParsedItem::from_parts(input, value))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_n_bytes() {
        assert_eq!(
            n_bytes(3).parse(b"hello").map(ParsedItem::into_parts),
            Ok((b"lo".as_ref(), b"hel".as_ref()))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_n_bytes_end_of_input() {
        assert_eq!(n_bytes(3).parse(b"he"), Err(error::EndOfInput));
    }
}
