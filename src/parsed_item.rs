#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedItem<'input, T> {
    input: &'input [u8],
    value: T,
}

impl<'input, T> ParsedItem<'input, T> {
    pub const fn from_parts(input: &'input [u8], value: T) -> Self {
        ParsedItem { input, value }
    }

    pub const fn input(&self) -> &'input [u8] {
        self.input
    }

    pub const fn value(&self) -> &T {
        &self.value
    }

    pub fn into_value(self) -> T {
        self.value
    }

    pub fn into_parts(self) -> (&'input [u8], T) {
        (self.input, self.value)
    }

    pub fn map_value<F, U>(self, f: F) -> ParsedItem<'input, U>
    where
        F: FnOnce(T) -> U,
    {
        ParsedItem::from_parts(self.input, f(self.value))
    }

    pub fn filter_map_value<F, U, E>(self, f: F) -> Result<ParsedItem<'input, U>, E>
    where
        F: FnOnce(T) -> Result<U, E>,
    {
        f(self.value).map(|value| ParsedItem::from_parts(self.input, value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ITEM: ParsedItem<'_, u8> = ParsedItem::from_parts(b"abc", 0);

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_parsed_item_input() {
        assert_eq!(ITEM.input(), b"abc");
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_parsed_item_value() {
        assert_eq!(ITEM.value(), &0);
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_parsed_item_into_value() {
        assert_eq!(ITEM.into_value(), 0);
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_parsed_item_into_parts() {
        assert_eq!(ITEM.into_parts(), (b"abc".as_ref(), 0));
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn test_parsed_item_map_value() {
        assert_eq!(ITEM.map_value(|v| v + 1).into_value(), 1);
    }
}
