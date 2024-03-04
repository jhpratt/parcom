use crate::{Combinator, ParsedItem, Parser};

pub const fn discard_at_least_n<'input, P>(
    n: usize,
) -> impl Combinator<'input, P, Output = usize, Error = P::Error> + Copy
where
    P: Parser<'input>,
{
    move |parser: P| {
        move |mut input| {
            let mut count = 0;

            loop {
                match parser.parse(input) {
                    Ok(parsed_item) => {
                        count += 1;
                        input = parsed_item.input();
                    }
                    Err(err) if count < n => return Err(err),
                    Err(_) => {
                        return Ok(ParsedItem::from_parts(input, count));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ascii;
    use crate::{error, Either, ParsedItem, Parser as _};

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_discard_at_least_n_success() {
        let parser = ascii::alphabetic.discard_at_least_n(2);

        assert_eq!(
            parser.parse(b"ab").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 2))
        );
        assert_eq!(
            parser.parse(b"abc").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), 3))
        );
        assert_eq!(
            parser.parse(b"abc0").map(ParsedItem::into_parts),
            Ok((b"0".as_ref(), 3))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_discard_at_least_n_error() {
        let parser = ascii::alphabetic.discard_at_least_n(2);

        assert_eq!(parser.parse(b""), Err(Either::B(error::EndOfInput)));
        assert_eq!(
            parser.parse(b"a").map(ParsedItem::into_parts),
            Err(Either::B(error::EndOfInput))
        );
    }
}
