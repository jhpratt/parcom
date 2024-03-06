use core::convert::Infallible;

use crate::{Combinator, ParsedItem, Parser};

pub const fn at_most_n_raw<'input, P>(
    n: usize,
) -> impl Combinator<'input, P, Output = &'input [u8], Error = Infallible> + Copy
where
    P: Parser<'input>,
{
    move |parser: P| {
        move |mut input| {
            let orig_input = input;

            for _ in 0..n {
                let Ok(parsed_item) = parser.parse(input) else {
                    let output = &orig_input[..(orig_input.len() - input.len())];
                    return Ok(ParsedItem::from_parts(input, output));
                };
                input = parsed_item.input();
            }

            let output = &orig_input[..(orig_input.len() - input.len())];
            Ok(ParsedItem::from_parts(input, output))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::any_byte;
    use crate::{ParsedItem, Parser as _};

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_at_most_n_raw() {
        let parser = any_byte.filter(u8::is_ascii_alphabetic).at_most_n_raw(2);

        assert_eq!(
            parser.parse(b"a").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b"a".as_ref()))
        );
        assert_eq!(
            parser.parse(b"ab").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b"ab".as_ref()))
        );
        assert_eq!(
            parser.parse(b"abc").map(ParsedItem::into_parts),
            Ok((b"c".as_ref(), b"ab".as_ref()))
        );
        assert_eq!(
            parser.parse(b"ab0").map(ParsedItem::into_parts),
            Ok((b"0".as_ref(), b"ab".as_ref()))
        );
    }
}
