use alloc::vec::Vec;

use crate::{Combinator, ParsedItem, Parser};

pub const fn n_to_m<'input, P>(
    n: usize,
    m: usize,
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

            for _ in n..m {
                let Ok(parsed_item) = parser.parse(input) else {
                    break;
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
    use super::*;
    use crate::parser::byte;
    use crate::{error, Either};

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_n_to_m_valid() {
        assert_eq!(
            byte(b'a')
                .n_to_m(2, 4)
                .parse(b"aa")
                .map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b"aa".to_vec()))
        );
        assert_eq!(
            byte(b'a')
                .n_to_m(2, 4)
                .parse(b"aaa")
                .map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b"aaa".to_vec()))
        );
        assert_eq!(
            byte(b'a')
                .n_to_m(2, 4)
                .parse(b"aaaa")
                .map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b"aaaa".to_vec()))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_n_to_m_invalid() {
        assert_eq!(
            byte(b'a')
                .n_to_m(2, 4)
                .parse(b"b")
                .map(ParsedItem::into_parts),
            Err(Either::A(error::Byte))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_n_to_m_end_of_input() {
        assert_eq!(
            byte(b'a')
                .n_to_m(2, 4)
                .parse(b"a")
                .map(ParsedItem::into_parts),
            Err(Either::B(error::EndOfInput))
        );
    }
}
