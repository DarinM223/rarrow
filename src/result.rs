use crate::classes::*;
use crate::mirror::{Hkt1, Hkt2, Mirror1, Mirror2};
use core::marker::PhantomData;
use std::fmt::Debug;

impl<A, E> Mirror1 for Result<A, E> {
    type T = A;
    type Family = ResultFamily1<E>;
}

impl<A, B> Mirror2 for Result<A, B> {
    type A = A;
    type B = B;
    type Family = ResultFamily2;
}

pub struct ResultFamily1<E> {
    e: PhantomData<E>,
}

impl<E> Hkt1 for ResultFamily1<E> {
    type Member<T> = Result<T, E>;
}

pub struct ResultFamily2;

impl Hkt2 for ResultFamily2 {
    type Member<A, B> = Result<A, B>;
}

impl<E> Functor for ResultFamily1<E> {
    fn fmap<A, B, F: Fn(A) -> B>(f: F, fa: Result<A, E>) -> Result<B, E> {
        fa.map(f)
    }
}

impl<E> Apply for ResultFamily1<E> {
    fn ap<A, B, F: Fn(A) -> B>(fa: Result<F, E>, fb: Result<A, E>) -> Result<B, E> {
        match (fa, fb) {
            (Ok(f), Ok(v)) => Ok(f(v)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }
}

impl<E> Applicative for ResultFamily1<E> {
    fn pure<A>(a: A) -> Result<A, E> {
        Ok(a)
    }
}

impl<E> Monad for ResultFamily1<E> {
    fn bind<A, B, F: Fn(A) -> Result<B, E>>(fa: Result<A, E>, f: F) -> Result<B, E> {
        fa.and_then(f)
    }
}

impl<E> Traversable for ResultFamily1<E> {
    fn traverse<App: Applicative, A, B, F: Fn(A) -> App::Member<B>>(
        f: F,
        t: Result<A, E>,
    ) -> App::Member<Result<B, E>> {
        match t {
            Ok(v) => App::fmap(|v| Ok(v), f(v)),
            Err(e) => App::pure(Err(e)),
        }
    }
}

impl<E> Foldable for ResultFamily1<E> {
    fn fold_left<A, B, F: Fn(B, A) -> B>(f: F, init: B, t: Result<A, E>) -> B {
        match t {
            Ok(a) => f(init, a),
            Err(_) => init,
        }
    }

    fn fold_right<A, B, F: Fn(A, B) -> B>(f: F, init: B, t: Result<A, E>) -> B {
        ResultFamily1::fold_left(|b, a| f(a, b), init, t)
    }
}

impl<E> SemigroupK for ResultFamily1<E> {
    fn combine_k<A>(fa: Result<A, E>, fb: Result<A, E>) -> Result<A, E> {
        match (fa, fb) {
            (Err(_), b) => b,
            (a, _) => a,
        }
    }
}

impl Bifunctor for ResultFamily2 {
    fn bimap<A, B, C, D, F1: Fn(A) -> B, F2: Fn(C) -> D>(
        f1: F1,
        f2: F2,
        f: Result<A, C>,
    ) -> Result<B, D> {
        match f {
            Ok(v) => Ok(f1(v)),
            Err(e) => Err(f2(e)),
        }
    }
}

impl<A, E> Semigroup for Result<A, E> {
    fn combine(self, other: Result<A, E>) -> Result<A, E> {
        match self {
            Err(_) => other,
            _ => self,
        }
    }
}

impl<A: Debug, E: Debug> Show for Result<A, E> {
    fn show(a: Result<A, E>) -> String {
        format!("{:?}", a)
    }
}

#[cfg(test)]
mod tests {
    use crate::classes::*;

    #[test]
    fn test_semigroup() {
        assert_eq!(Semigroup::combine(Ok::<_, i32>(1), Ok(2)), Ok(1));
    }
}
