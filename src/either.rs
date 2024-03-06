use core::convert::Infallible;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Either<A, B> {
    A(A),
    B(B),
}

impl<A> Either<A, Infallible> {
    pub fn into_a(self) -> A {
        match self {
            Self::A(a) => a,
            Self::B(b) => match b {},
        }
    }
}

impl<B> Either<Infallible, B> {
    pub fn into_b(self) -> B {
        match self {
            Self::A(a) => match a {},
            Self::B(b) => b,
        }
    }
}

impl<A> Either<A, A> {
    pub fn unify(self) -> A {
        match self {
            Self::A(value) | Self::B(value) => value,
        }
    }
}

impl<A, B> Either<A, B> {
    pub fn map_a<T, F>(self, f: F) -> Either<T, B>
    where
        F: FnOnce(A) -> T,
    {
        match self {
            Self::A(a) => Either::A(f(a)),
            Self::B(b) => Either::B(b),
        }
    }

    pub fn map_b<T, F>(self, f: F) -> Either<A, T>
    where
        F: FnOnce(B) -> T,
    {
        match self {
            Self::A(a) => Either::A(a),
            Self::B(b) => Either::B(f(b)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn into_a() {
        assert_eq!(Either::<u8, Infallible>::A(0).into_a(), 0);
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn into_b() {
        assert_eq!(Either::<Infallible, u8>::B(0).into_b(), 0);
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn unify() {
        assert_eq!(Either::<u8, u8>::A(0).unify(), 0);
        assert_eq!(Either::<u8, u8>::B(1).unify(), 1);
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn map_a() {
        #[cfg_attr(coverage, coverage(off))]
        const fn f(x: u8) -> u8 {
            x + 2
        }

        assert_eq!(Either::<u8, u8>::A(0).map_a(f), Either::A(2));
        assert_eq!(Either::<u8, u8>::B(1).map_a(f), Either::B(1));
    }

    #[test]
    #[cfg_attr(coverage, coverage(off))]
    fn map_b() {
        #[cfg_attr(coverage, coverage(off))]
        const fn f(x: u8) -> u8 {
            x + 2
        }

        assert_eq!(Either::<u8, u8>::A(0).map_b(f), Either::A(0));
        assert_eq!(Either::<u8, u8>::B(1).map_b(f), Either::B(3));
    }
}
