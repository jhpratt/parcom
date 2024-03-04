//! `parcom` is a parser combinator library.

#![cfg_attr(__parcom_01_docs, feature(doc_auto_cfg))]
#![cfg_attr(coverage, feature(coverage_attribute))]
#![cfg_attr(not(feature = "std"), no_std)]
#![doc(test(attr(deny(warnings))))]
// temporary
#![allow(missing_docs, clippy::missing_docs_in_private_items)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod combinator;
pub mod error;
mod parsed_item;
pub mod parser;

#[doc(inline)]
pub use self::combinator::Combinator;
#[doc(inline)]
pub use self::error::Error;
pub use self::parsed_item::ParsedItem;
#[doc(inline)]
pub use self::parser::{Parser, ParserResult};

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Either<A, B> {
    A(A),
    B(B),
}

#[macro_export]
macro_rules! seq {
    () => {};
    ($first:pat $(,)?) => {
        $first
    };
    ($first:pat, $second:pat $(, $($rest:pat),+ $(,)?)?) => {
        $crate::seq!(($first, $second), $($($rest),+)?)
    };
}

/// When attempting to return `impl Parser` from a function, a lifetime is necessary. If this
/// lifetime is entirely up to the caller, a higher ranked trait bound (HRTB) should be used. The
/// compiler can be quite bad at inferring lifetimes of parameters to closures (namely the input),
/// but wrapping the closure in this function will force the compiler to infer the lifetime. It's
/// not perfect, but it's better than nothing.
///
/// One limitation appears to be when the output type contains a reference to the input. Inference
/// still fails in this case. When this occurs, it is necessary to make a copy of this function with
/// the relevant generics made concrete. For some reason this works. This situation is seen with the
/// `n_bytes` parser.
const fn hrtb_hack<Output, Error, F>(f: F) -> F
where
    F: Fn(&[u8]) -> ParserResult<'_, Output, Error>,
{
    f
}
