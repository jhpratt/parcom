use alloc::vec::Vec;

use crate::{Combinator, ParsedItem, Parser};

pub const fn at_least_n<'input, P>(
    n: usize,
) -> impl Combinator<'input, P, Output = Vec<P::Output>, Error = P::Error> + Copy
where
    P: Parser<'input>,
{
    move |parser: P| {
        move |mut input| {
            let mut output = Vec::new();

            for _ in 0..n {
                let parsed_item = parser.parse(input)?;
                let (remaining_input, value) = parsed_item.into_parts();
                input = remaining_input;
                output.push(value);
            }

            loop {
                match parser.parse(input) {
                    Ok(parsed_item) => {
                        let (remaining_input, value) = parsed_item.into_parts();
                        input = remaining_input;
                        output.push(value);
                    }
                    Err(_) => {
                        return Ok(ParsedItem::from_parts(input, output));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::any_byte;
    use crate::{error, Either, ParsedItem, Parser as _};

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_at_least_n_success() {
        let parser = any_byte.filter(u8::is_ascii_alphabetic).at_least_n(2);

        assert_eq!(
            parser.parse(b"abc").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b"abc".to_vec()))
        );
        assert_eq!(
            parser.parse(b"ab0").map(ParsedItem::into_parts),
            Ok((b"0".as_ref(), b"ab".to_vec()))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_at_least_n_error() {
        let parser = any_byte.filter(u8::is_ascii_alphabetic).at_least_n(2);
        assert_eq!(parser.parse(b""), Err(Either::B(error::EndOfInput)));
    }
}
