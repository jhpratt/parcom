use core::convert::Infallible;

use crate::{Combinator, ParsedItem, Parser};

/// Discard items from the input while the predicate returns `true`.
///
/// The output of this combinator is a tuple containing the number of items discarded and either the
/// item that caused the predicate to return `false` or a parse error. This combinator will never
/// fail.
pub const fn discard_while<'input, F, P>(
    f: F,
) -> impl Combinator<'input, P, Output = usize, Error = Infallible> + Copy
where
    P: Parser<'input>,
    F: Fn(&P::Output) -> bool + Copy,
{
    move |parser: P| {
        move |mut input| {
            let mut count = 0;

            loop {
                match parser.parse(input) {
                    Ok(parsed_item) if f(parsed_item.value()) => {
                        count += 1;
                        input = parsed_item.input();
                    }
                    Ok(_) | Err(_) => {
                        return Ok(ParsedItem::from_parts(input, count));
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
    fn test_discard_while() {
        let parser = any_byte.discard_while(
            #[cfg_attr(coverage, coverage(off))]
            |&byte| byte == b'a',
        );

        assert_eq!(
            parser.parse(b"").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 0))
        );
        assert_eq!(
            parser.parse(b"a").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 1))
        );
        assert_eq!(
            parser.parse(b"ab").map(ParsedItem::into_parts),
            Ok((b"b".as_ref(), 1))
        );
    }
}
