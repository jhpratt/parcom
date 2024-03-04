use crate::{error, ParsedItem, ParserResult};

pub fn end_of_input(input: &[u8]) -> ParserResult<'_, (), error::NotEndOfInput> {
    input
        .is_empty()
        .then_some(ParsedItem::from_parts(input, ()))
        .ok_or(error::NotEndOfInput)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_end_of_input() {
        assert_eq!(
            end_of_input.parse(b"").map(ParsedItem::into_parts),
            Ok((b"".as_ref(), ()))
        );
        assert_eq!(end_of_input.parse(b"a"), Err(error::NotEndOfInput));
    }
}
