use crate::{Combinator, ParsedItem, Parser};

pub const fn n_to_m_raw<'input, P>(
    n: usize,
    m: usize,
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

            for _ in n..m {
                let Ok(parsed_item) = parser.parse(input) else {
                    break;
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
    use super::*;
    use crate::parser::byte;
    use crate::{error, Either};

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_n_to_m_raw_valid() {
        assert_eq!(
            byte(b'a')
                .n_to_m_raw(2, 4)
                .parse(b"aa")
                .map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b"aa".as_ref()))
        );
        assert_eq!(
            byte(b'a')
                .n_to_m_raw(2, 4)
                .parse(b"aaa")
                .map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b"aaa".as_ref()))
        );
        assert_eq!(
            byte(b'a')
                .n_to_m_raw(2, 4)
                .parse(b"aaaa")
                .map(ParsedItem::into_parts),
            Ok((b"".as_ref(), b"aaaa".as_ref()))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_n_to_m_raw_invalid() {
        assert_eq!(
            byte(b'a')
                .n_to_m_raw(2, 4)
                .parse(b"b")
                .map(ParsedItem::into_parts),
            Err(Either::A(error::Byte))
        );
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_n_to_m_raw_end_of_input() {
        assert_eq!(
            byte(b'a')
                .n_to_m_raw(2, 4)
                .parse(b"a")
                .map(ParsedItem::into_parts),
            Err(Either::B(error::EndOfInput))
        );
    }
}
