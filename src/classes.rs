use crate::plug::{Plug, Plug2, Unplug, Unplug2};

pub trait Functor: Unplug + Plug<<Self as Unplug>::A> {
    fn map<B, F>(self, f: F) -> <Self as Plug<B>>::Out
    where
        Self: Plug<B>,
        F: FnOnce(<Self as Unplug>::A) -> B;
}

pub trait Apply: Functor {
    fn ap<B, F>(self, f: <Self as Plug<F>>::Out) -> <Self as Plug<B>>::Out
    where
        Self: Plug<B> + Plug<F>,
        F: FnOnce(<Self as Unplug>::A) -> B;
}

pub trait Applicative: Apply {
    fn pure(value: <Self as Unplug>::A) -> Self;
}

pub trait Monad: Applicative {
    fn ret(value: <Self as Unplug>::A) -> Self;
    fn bind<B, F>(self, f: F) -> <Self as Plug<B>>::Out
    where
        Self: Plug<B>,
        F: FnOnce(<Self as Unplug>::A) -> <Self as Plug<B>>::Out;
}

pub trait Semigroup: Unplug + Plug<<Self as Unplug>::A> {
    fn combine(a: <Self as Unplug>::A, b: <Self as Unplug>::A) -> <Self as Unplug>::A;
}

pub trait Monoid: Semigroup {
    fn mempty() -> <Self as Unplug>::A;
    fn mappend(a: <Self as Unplug>::A, b: <Self as Unplug>::A) -> <Self as Unplug>::A;
}

pub trait Foldable: Unplug + Plug<<Self as Unplug>::A> {
    fn fold_left<B, F>(self, init: B, f: F) -> B
    where
        F: FnOnce(B, <Self as Unplug>::A) -> B;
}

pub trait Traverse: Functor + Foldable {
    fn traverse<F, M, B>(self, f: F) -> <M as Plug<<Self as Plug<B>>::Out>>::Out
    where
        Self: Plug<B>,
        M: Plug<<Self as Plug<B>>::Out> + Plug<B> + Applicative,
        F: FnOnce(<Self as Unplug>::A) -> <M as Plug<B>>::Out;
}

pub trait Eq: Unplug + Plug<<Self as Unplug>::A> {
    fn eq(a: <Self as Unplug>::A, b: <Self as Unplug>::A) -> bool;
}

pub trait Show: Unplug + Plug<<Self as Unplug>::A> {
    fn show(a: <Self as Unplug>::A) -> String;
}

pub trait Contravariant: Unplug + Plug<<Self as Unplug>::A> {
    fn contramap<B, F>(self, f: F) -> <Self as Plug<B>>::Out
    where
        Self: Plug<B>,
        F: FnOnce(B) -> <Self as Unplug>::A;
}

pub trait Alternative: Applicative {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

pub trait Bifunctor: Unplug2 + Plug2<<Self as Unplug2>::A, <Self as Unplug2>::B> {
    fn bimap<C, D, F, G>(self, f: F, g: G) -> <Self as Plug2<C, D>>::Out
    where
        Self: Plug2<C, D>,
        F: FnOnce(<Self as Unplug2>::A) -> C,
        G: FnOnce(<Self as Unplug2>::B) -> D;
}
