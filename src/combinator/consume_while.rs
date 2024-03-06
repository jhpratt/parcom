#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::convert::Infallible;

use crate::{Combinator, ParsedItem, Parser};

/// Consume items from the input while the predicate returns `true`.
///
/// The output of this combinator is a tuple containing the number of items discarded and either the
/// item that caused the predicate to return `false` or a parse error. This combinator will never
/// fail.
pub const fn consume_while<'input, P, F>(
    f: F,
) -> impl Combinator<'input, P, Output = Vec<P::Output>, Error = Infallible> + Copy
where
    P: Parser<'input>,
    F: Fn(&P::Output) -> bool + Copy,
{
    move |parser: P| {
        move |mut input| {
            let mut output = Vec::new();

            loop {
                match parser.parse(input) {
                    Ok(parsed_item) if f(parsed_item.value()) => {
                        let (remaining_input, value) = parsed_item.into_parts();
                        input = remaining_input;
                        output.push(value);
                    }
                    Ok(_) => {
                        return Ok(ParsedItem::from_parts(input, output));
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
    use crate::{ParsedItem, Parser as _};

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_consume_while() {
        let parser = any_byte.consume_while(u8::is_ascii_alphabetic);

        assert_eq!(
            parser.parse(b"abc").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b"abc".to_vec()))
        );
        assert_eq!(
            parser.parse(b"ab0").map(ParsedItem::into_parts),
            Ok((b"0".as_ref(), b"ab".to_vec()))
        );
    }
}
