#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::convert::Infallible;

use crate::{Combinator, ParsedItem, Parser};

pub const fn at_most_n<'input, P>(
    n: usize,
) -> impl Combinator<'input, P, Output = Vec<P::Output>, Error = Infallible> + Copy
where
    P: Parser<'input>,
{
    move |parser: P| {
        move |mut input| {
            let mut output = Vec::new();

            for _ in 0..n {
                let Ok(parsed_item) = parser.parse(input) else {
                    return Ok(ParsedItem::from_parts(input, output));
                };
                let (remaining_input, value) = parsed_item.into_parts();
                input = remaining_input;
                output.push(value);
            }

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
    fn test_at_most_n() {
        let parser = any_byte.filter(u8::is_ascii_alphabetic).at_most_n(2);

        assert_eq!(
            parser.parse(b"a").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b"a".to_vec()))
        );
        assert_eq!(
            parser.parse(b"ab").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b"ab".to_vec()))
        );
        assert_eq!(
            parser.parse(b"abc").map(ParsedItem::into_parts),
            Ok((b"c".as_ref(), b"ab".to_vec()))
        );
        assert_eq!(
            parser.parse(b"ab0").map(ParsedItem::into_parts),
            Ok((b"0".as_ref(), b"ab".to_vec()))
        );
    }
}
