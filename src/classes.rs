use crate::plug::{Plug, Unplug};

pub trait Functor: Unplug + Plug<<Self as Unplug>::A> {
    fn map<B, F>(self, f: F) -> <Self as Plug<B>>::Out
    where
        Self: Plug<B>,
        F: FnMut(<Self as Unplug>::A) -> B;
}

pub trait Applicative: Functor {
    fn pure(value: <Self as Unplug>::A) -> Self;
    fn ap<B, F>(self, f: <Self as Plug<F>>::Out) -> <Self as Plug<B>>::Out
    where
        Self: Plug<B> + Plug<F>,
        F: FnOnce(<Self as Unplug>::A) -> B;
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
    fn fold_right<B, F>(self, init: B, f: F) -> B
    where
        F: FnOnce(<Self as Unplug>::A, B) -> B;
}

pub trait Traverse: Unplug + Plug<<Self as Unplug>::A> {
    fn traverse<F, M, B>(self, f: F) -> <M as Plug<<Self as Plug<B>>::Out>>::Out
    where
        Self: Plug<B>,
        M: Plug<<Self as Plug<B>>::Out> + Plug<B> + Applicative,
        F: FnOnce(<Self as Unplug>::A) -> <M as Plug<B>>::Out;
}
