use crate::Parser;

pub trait Combinator<'input, P>
where
    P: Parser<'input>,
{
    type Output;
    type Error;

    fn apply_to(
        &self,
        parser: P,
    ) -> impl Parser<'input, Output = Self::Output, Error = Self::Error>;
}

impl<'input, P, F, FRet, Output, Error> Combinator<'input, P> for F
where
    F: Fn(P) -> FRet,
    FRet: Parser<'input, Output = Output, Error = Error>,
    P: Parser<'input>,
{
    type Output = Output;
    type Error = Error;

    fn apply_to(
        &self,
        parser: P,
    ) -> impl Parser<'input, Output = Self::Output, Error = Self::Error> {
        self(parser)
    }
}
