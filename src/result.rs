use crate::classes::*;
use crate::plug::{Plug, Plug2, Unplug, Unplug2};
use std::fmt::Debug;

impl<A, E> Unplug for Result<A, E> {
    type F = Result<A, E>;
    type A = A;
}

impl<A, B, E> Plug<B> for Result<A, E> {
    type Out = Result<B, E>;
}

impl<A, B> Unplug2 for Result<A, B> {
    type F = Result<A, B>;
    type A = A;
    type B = B;
}

impl<A, B, C, D> Plug2<C, D> for Result<A, B> {
    type Out = Result<C, D>;
}

impl<A, E> Functor for Result<A, E> {
    fn fmap<B, F: FnMut(A) -> B>(self, f: F) -> <Self as Plug<B>>::Out {
        self.map(f)
    }
}

impl<A, E> Apply for Result<A, E> {
    fn ap<B, F: FnMut(A) -> B>(self, f: <Self as Plug<F>>::Out) -> <Self as Plug<B>>::Out {
        f.and_then(|fun| self.map(fun))
    }
}

impl<A, E> Applicative for Result<A, E> {
    fn pure(value: A) -> Self {
        Ok(value)
    }
}

impl<A, E> Monad for Result<A, E> {
    fn bind<B, F>(self, f: F) -> <Self as Plug<B>>::Out
    where
        F: FnMut(A) -> <Self as Plug<B>>::Out,
    {
        self.and_then(f)
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

impl<A, E> Foldable for Result<A, E> {
    fn fold_left<B, F: FnMut(B, A) -> B>(self, init: B, mut f: F) -> B {
        match self {
            Ok(a) => f(init, a),
            Err(_) => init,
        }
    }

    fn fold_right<B, F: FnMut(A, B) -> B>(self, init: B, mut f: F) -> B {
        self.fold_left(init, |b, a| f(a, b))
    }
}

impl<A: Debug, E: Debug> Show for Result<A, E> {
    fn show(a: Result<A, E>) -> String {
        format!("{:?}", a)
    }
}

impl<A, E> SemigroupK for Result<A, E> {
    fn combine_k(self, other: Result<A, E>) -> Result<A, E> {
        match (self, other) {
            (Err(_), b) => b,
            (a, _) => a,
        }
    }
}

// This also doesn't work...
// It can't infer that Result<C, D> == <Result<A, E> as Plug2<C, D>>::Out

// impl<A, E> Bifunctor for Result<A, E> {
//     fn bimap<C, D, F, G>(self, f: F, g: G) -> <Self as Plug2<C, D>>::Out
//     where
//         Self: Plug2<C, D>,
//         F: FnOnce(A) -> C,
//         G: FnOnce(E) -> D
//     {
//         match self {
//             Ok(a) => Ok(f(a)) as Result<C, D>,
//             Err(e) => Err(g(e)) as Result<C, D>,
//         }
//     }
// }
