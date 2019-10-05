use crate::plug::{Plug, Plug2, Unplug, Unplug2};

pub trait Functor: Unplug + Plug<<Self as Unplug>::A> {
    fn fmap<B, F>(self, f: F) -> <Self as Plug<B>>::Out
    where
        Self: Plug<B>,
        F: FnMut(<Self as Unplug>::A) -> B;
}

pub trait Apply: Functor {
    fn ap<B, F>(self, f: <Self as Plug<F>>::Out) -> <Self as Plug<B>>::Out
    where
        Self: Plug<B> + Plug<F>,
        F: Fn(<Self as Unplug>::A) -> B;
}

pub trait Applicative: Apply {
    fn pure(value: <Self as Unplug>::A) -> Self;
}

pub trait Monad: Applicative {
    fn bind<B, F>(self, f: F) -> <Self as Plug<B>>::Out
    where
        Self: Plug<B>,
        F: FnMut(<Self as Unplug>::A) -> <Self as Plug<B>>::Out;
}

pub trait Semigroup {
    fn combine(self, other: Self) -> Self;
}

pub trait Monoid: Semigroup {
    fn mempty() -> Self;
}

pub trait Foldable: Unplug + Plug<<Self as Unplug>::A> {
    fn fold_left<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, <Self as Unplug>::A) -> B;

    fn fold_right<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(<Self as Unplug>::A, B) -> B;
}

pub trait Traverse: Functor + Foldable {
    fn traverse<F, M, B>(self, f: F) -> <M as Plug<<Self as Plug<B>>::Out>>::Out
    where
        Self: Plug<B>,
        M: Unplug<A = B> + Plug<<Self as Plug<B>>::Out>,
        <M as Plug<<Self as Plug<B>>::Out>>::Out: Applicative,
        F: FnOnce(<Self as Unplug>::A) -> M;
}

pub trait Show {
    fn show(a: Self) -> String;
}

pub trait Contravariant: Unplug + Plug<<Self as Unplug>::A> {
    fn contramap<B, F>(self, f: F) -> <Self as Plug<B>>::Out
    where
        Self: Plug<B>,
        F: FnOnce(B) -> <Self as Unplug>::A;
}

pub trait SemigroupK: Unplug + Plug<<Self as Unplug>::A> {
    fn combine_k(self, other: Self) -> Self;
}

pub trait MonoidK: SemigroupK {
    fn empty() -> Self;
}

pub trait Alternative: Applicative + MonoidK {}

pub trait Bifunctor: Unplug2 + Plug2<<Self as Unplug2>::A, <Self as Unplug2>::B> {
    fn bimap<C, D, F, G>(self, f: F, g: G) -> <Self as Plug2<C, D>>::Out
    where
        Self: Plug2<C, D>,
        F: FnOnce(<Self as Unplug2>::A) -> C,
        G: FnOnce(<Self as Unplug2>::B) -> D;
}
