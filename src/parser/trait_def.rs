#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::convert::Infallible;

use crate::combinator::{
    and, discard, discard_at_least_n, discard_while, filter, filter_map, map, map_err, optional,
    or, Combinator,
};
#[cfg(feature = "alloc")]
use crate::combinator::{at_least_n, consume_while, exactly_n, n_to_m};
use crate::error::Filter;
use crate::{Either, ParserResult};

/// A type that is capable of consuming input and producing a parsed item.
pub trait Parser<'input>: Copy {
    type Output;
    type Error;

    fn parse(self, input: &'input [u8]) -> ParserResult<'input, Self::Output, Self::Error>;

    fn with<C>(self, combinator: &C) -> impl Parser<'input, Output = C::Output, Error = C::Error>
    where
        C: Combinator<'input, Self>,
    {
        combinator.apply_to(self)
    }

    // Helper methods for parcom-provided combinators.

    #[rustfmt::skip] // rust-lang/rustfmt#3599
    fn and<P2>(
        self,
        other: P2,
    ) -> impl Parser<
        'input,
        Output = (Self::Output, P2::Output),
        Error = Either<Self::Error, P2::Error>,
    >
    where
        P2: Parser<'input>,
    {
        move |input| self.with(&and(other)).parse(input)
    }

    #[cfg(feature = "alloc")]
    fn at_least_n(
        self,
        n: usize,
    ) -> impl Parser<'input, Output = Vec<Self::Output>, Error = Self::Error> {
        move |input| self.with(&at_least_n(n)).parse(input)
    }

    #[cfg(feature = "alloc")]
    fn consume_while<F>(
        self,
        f: F,
    ) -> impl Parser<'input, Output = Vec<Self::Output>, Error = Infallible>
    where
        F: Fn(&Self::Output) -> bool + Copy,
    {
        move |input| self.with(&consume_while(f)).parse(input)
    }

    fn discard_at_least_n(
        self,
        n: usize,
    ) -> impl Parser<'input, Output = usize, Error = Self::Error> {
        move |input| self.with(&discard_at_least_n(n)).parse(input)
    }

    fn discard_while<F>(self, f: F) -> impl Parser<'input, Output = usize, Error = Infallible>
    where
        F: Fn(&Self::Output) -> bool + Copy,
    {
        move |input| self.with(&discard_while(f)).parse(input)
    }

    fn discard(self) -> impl Parser<'input, Output = (), Error = Self::Error> {
        move |input| self.with(&discard()).parse(input)
    }

    #[cfg(feature = "alloc")]
    fn exactly_n(
        self,
        n: usize,
    ) -> impl Parser<'input, Output = Vec<Self::Output>, Error = Self::Error> {
        move |input| self.with(&exactly_n(n)).parse(input)
    }

    fn filter<F>(
        self,
        f: F,
    ) -> impl Parser<'input, Output = Self::Output, Error = Either<Filter, Self::Error>>
    where
        F: Fn(&Self::Output) -> bool + Copy,
    {
        move |input| self.with(&filter(f)).parse(input)
    }

    fn filter_map<F, U, E>(
        self,
        f: F,
    ) -> impl Parser<'input, Output = U, Error = Either<E, Self::Error>>
    where
        F: Fn(Self::Output) -> Result<U, E> + Copy,
    {
        move |input| self.with(&filter_map(f)).parse(input)
    }

    fn map<F, NewOutput>(self, f: F) -> impl Parser<'input, Output = NewOutput, Error = Self::Error>
    where
        F: Fn(Self::Output) -> NewOutput + Copy,
    {
        move |input| self.with(&map(f)).parse(input)
    }

    fn map_err<F, NewError>(
        self,
        f: F,
    ) -> impl Parser<'input, Output = Self::Output, Error = NewError>
    where
        F: Fn(Self::Error) -> NewError + Copy,
    {
        move |input| self.with(&map_err(f)).parse(input)
    }

    #[cfg(feature = "alloc")]
    fn n_to_m(
        self,
        n: usize,
        m: usize,
    ) -> impl Parser<'input, Output = Vec<Self::Output>, Error = Self::Error> {
        move |input| self.with(&n_to_m(n, m)).parse(input)
    }

    fn optional(self) -> impl Parser<'input, Output = Option<Self::Output>, Error = Infallible> {
        move |input| self.with(&optional()).parse(input)
    }

    #[rustfmt::skip] // rust-lang/rustfmt#3599
    fn or<P2>(
        self,
        other: P2,
    ) -> impl Parser<
        'input,
        Output = Either<Self::Output, P2::Output>,
        Error = (Self::Error, P2::Error),
    >
    where
        P2: Parser<'input>,
    {
        move |input| self.with(&or(other)).parse(input)
    }
}

impl<'input, F, Output, Error> Parser<'input> for F
where
    F: Fn(&'input [u8]) -> ParserResult<'input, Output, Error> + Copy,
{
    type Output = Output;
    type Error = Error;

    fn parse(self, input: &'input [u8]) -> ParserResult<'input, Self::Output, Self::Error> {
        self(input)
    }
}
