use core::convert::Infallible;

use crate::{hrtb_hack, ParsedItem, Parser};

pub fn success<T>(value: T) -> impl for<'input> Parser<'input, Output = T, Error = Infallible>
where
    T: Copy,
{
    hrtb_hack(move |input| Ok(ParsedItem::from_parts(input, value)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;

    #[test]
    fn test_success() {
        assert_eq!(
            success(()).parse(b"abc").map(ParsedItem::into_parts),
            Ok((b"abc".as_ref(), ()))
        );
    }
}
