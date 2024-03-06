use crate::{Combinator, Parser};

pub const fn inspect_err<'input, P, F>(
    f: F,
) -> impl Combinator<'input, P, Output = P::Output, Error = P::Error> + Copy
where
    P: Parser<'input>,
    F: Fn(&P::Error) + Copy,
{
    move |parser: P| {
        move |input| match parser.parse(input) {
            Ok(parsed) => Ok(parsed),
            Err(err) => {
                f(&err);
                Err(err)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use core::cell::Cell;

    use crate::parser::byte;
    use crate::{error, Either, ParsedItem, Parser as _};

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_inspect_err() {
        // `Cell` is needed to make the closure `Copy`.
        let val = Cell::new(0);
        let parser = byte(b'a').inspect_err(
            #[cfg_attr(coverage, coverage(off))]
            |_| val.set(val.get() + 1),
        );

        assert_eq!(
            parser.parse(b"abc").map(ParsedItem::into_parts),
            Ok((b"bc".as_ref(), b'a'))
        );
        assert_eq!(val.get(), 0);
        assert_eq!(
            parser.parse(b"abc").map(ParsedItem::into_parts),
            Ok((b"bc".as_ref(), b'a'))
        );
        assert_eq!(val.get(), 0);
        assert_eq!(parser.parse(b"bc"), Err(Either::A(error::Byte)));
        assert_eq!(val.get(), 1);
    }
}
