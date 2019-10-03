use crate::classes::*;
use crate::plug::{Plug, Unplug};
use std::fmt::Debug;

impl<A> Unplug for Vec<A> {
    type F = Vec<A>;
    type A = A;
}

impl<A, B> Plug<B> for Vec<A> {
    type Out = Vec<B>;
}

impl<A> Functor for Vec<A> {
    fn fmap<B, F: FnMut(A) -> B>(self, f: F) -> <Self as Plug<B>>::Out {
        self.into_iter().map(f).collect()
    }
}

impl<A: Copy> Apply for Vec<A> {
    fn ap<B, F: FnMut(A) -> B>(self, fs: <Self as Plug<F>>::Out) -> <Self as Plug<B>>::Out {
        let mut result = Vec::with_capacity(self.len() * fs.len());
        for mut f in fs.into_iter() {
            for x in self.iter() {
                result.push(f(*x));
            }
        }
        result
    }
}

impl<A: Copy> Applicative for Vec<A> {
    fn pure(a: A) -> Self {
        vec![a]
    }
}

impl<A: Copy> Monad for Vec<A> {
    fn bind<B, F>(self, f: F) -> <Self as Plug<B>>::Out
    where
        F: FnMut(A) -> <Self as Plug<B>>::Out,
    {
        self.into_iter().flat_map(f).collect()
    }
}

impl<A> Semigroup for Vec<A> {
    fn combine(self, other: Vec<A>) -> Vec<A> {
        self.into_iter().chain(other.into_iter()).collect()
    }
}

impl<A> Monoid for Vec<A> {
    fn mempty() -> Vec<A> {
        vec![]
    }
}

impl<A> Foldable for Vec<A> {
    fn fold_left<B, F: FnMut(B, A) -> B>(self, init: B, f: F) -> B {
        self.into_iter().fold(init, f)
    }

    fn fold_right<B, F: FnMut(A, B) -> B>(self, init: B, mut f: F) -> B {
        self.into_iter().rev().fold(init, |a, b| f(b, a))
    }
}

impl<A: Debug> Show for Vec<A> {
    fn show(a: Vec<A>) -> String {
        format!("{:?}", a)
    }
}

impl<A: Copy> Alternative for Vec<A> {
    fn empty() -> Vec<A> {
        vec![]
    }

    fn combine_k(self, other: Vec<A>) -> Vec<A> {
        self.into_iter().chain(other.into_iter()).collect()
    }
}
