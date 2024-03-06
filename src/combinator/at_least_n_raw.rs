use crate::{Combinator, ParsedItem, Parser};

pub const fn at_least_n_raw<'input, P>(
    n: usize,
) -> impl Combinator<'input, P, Output = &'input [u8], Error = P::Error> + Copy
where
    P: Parser<'input>,
{
    move |parser: P| {
        move |mut input| {
            let orig_input = input;

            for _ in 0..n {
                let parsed_item = parser.parse(input)?;
                input = parsed_item.input();
            }

            loop {
                match parser.parse(input) {
                    Ok(parsed_item) => {
                        input = parsed_item.input();
                    }
                    Err(_) => {
                        let output = &orig_input[..(orig_input.len() - input.len())];
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
    fn test_at_least_n_raw_success() {
        let parser = any_byte.filter(u8::is_ascii_alphabetic).at_least_n_raw(2);

        assert_eq!(
            parser.parse(b"abc").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b"abc".as_ref()))
        );
        assert_eq!(
            parser.parse(b"ab0").map(ParsedItem::into_parts),
            Ok((b"0".as_ref(), b"ab".as_ref()))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_at_least_n_raw_error() {
        let parser = any_byte.filter(u8::is_ascii_alphabetic).at_least_n_raw(2);
        assert_eq!(parser.parse(b""), Err(Either::B(error::EndOfInput)));
    }
}
