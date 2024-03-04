use core::convert::Infallible;

use crate::Either;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Filter(Filter),
    EndOfInput(EndOfInput),
    AsciiDigit(AsciiDigit),
    AsciiAlphabetic(AsciiAlphabetic),
    AsciiAlphanumeric(AsciiAlphanumeric),
    AsciiWhitespace(AsciiWhitespace),
    AsciiHexDigit(AsciiHexDigit),
    Byte(Byte),
    NotEndOfInput(NotEndOfInput),
    NonMatchingInput(NonMatchingInput),
    Utf8Char(Utf8Char),
}

impl From<Infallible> for Error {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

impl<A, B> From<Either<A, B>> for Error
where
    A: Into<Self>,
    B: Into<Self>,
{
    fn from(e: Either<A, B>) -> Self {
        match e {
            Either::A(a) => a.into(),
            Either::B(b) => b.into(),
        }
    }
}

#[allow(unused_macro_rules)] // will be used in the future
macro_rules! declare_parcom_error {
    (@single
        $(#[$struct_attr:meta])*
        $vis:vis struct $name:ident;
    ) => {
        $(#[$struct_attr])*
        #[non_exhaustive]
        #[derive(Debug, Clone, PartialEq, Eq)]
        $vis struct $name;
    };
    (@single
        $(#[$struct_attr:meta])*
        $vis:vis struct $name:ident {$(
            $(#[$field_attr:meta])*
            $field_vis:vis $field_name:ident : $field_ty:ty
        ),* $(,)?}
    ) => {
        $(#[$struct_attr])*
        #[non_exhaustive]
        #[derive(Debug, Clone, PartialEq, Eq)]
        $vis struct $name {$(
            $(#[$field_attr])*
            $field_vis $field_name: $field_ty
        ),*}
    };
    ($(
        $(#[$struct_attr:meta])*
        $vis:vis struct $name:ident $fields:tt
    )*) => {
        $(declare_parcom_error! {
            @single
            $(#[$struct_attr])*
            $vis struct $name $fields
        })*

        $(impl From<$name> for Error {
            fn from(e: $name) -> Self {
                Self::$name(e)
            }
        })*

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            #[cfg_attr(coverage, coverage(off))]
            fn test_into_error_variants() {
                $(assert_eq!(Error::from($name), Error::$name($name));)*
            }

            #[test]
            #[cfg_attr(coverage, coverage(off))]
            fn test_into_error_either() {
                $(assert_eq!(Error::from(Either::<_, Filter>::A($name)), Error::$name($name));)*
                $(assert_eq!(Error::from(Either::<$name, _>::B(Filter)), Error::Filter(Filter));)*
                $(assert_eq!(Error::from(Either::<Infallible, _>::B($name)), Error::$name($name));)*
                $(assert_eq!(Error::from(Either::<_, $name>::A(Filter)), Error::Filter(Filter));)*
            }
        }
    };
}

declare_parcom_error! {
    pub struct Filter;
    /// The end of input was reached while trying to parse the value.
    pub struct EndOfInput;
    pub struct AsciiDigit;
    pub struct AsciiAlphabetic;
    pub struct AsciiAlphanumeric;
    pub struct AsciiWhitespace;
    pub struct AsciiHexDigit;
    pub struct Byte;
    pub struct NotEndOfInput;
    pub struct NonMatchingInput;
    pub struct Utf8Char;
}
