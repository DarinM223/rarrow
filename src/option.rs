use crate::classes::{Applicative, Apply, Foldable, Functor, Monad, Monoid, Semigroup};
use crate::plug::{Plug, Unplug};

impl<A> Unplug for Option<A> {
    type F = Option<A>;
    type A = A;
}

impl<A, B> Plug<B> for Option<A> {
    type Out = Option<B>;
}

impl<A> Functor for Option<A> {
    fn fmap<B, F>(self, f: F) -> <Self as Plug<B>>::Out
    where
        F: FnOnce(<Self as Unplug>::A) -> B,
    {
        self.map(f)
    }
}

impl<A> Apply for Option<A> {
    fn ap<B, F>(self, f: <Self as Plug<F>>::Out) -> <Self as Plug<B>>::Out
    where
        F: FnOnce(<Self as Unplug>::A) -> B,
    {
        match f {
            Some(fun) => self.map(fun),
            None => None,
        }
    }
}

impl<A> Applicative for Option<A> {
    fn pure(value: A) -> Self {
        Some(value)
    }
}

impl<A> Monad for Option<A> {
    fn bind<B, F>(self, f: F) -> <Self as Plug<B>>::Out
    where
        F: FnOnce(<Self as Unplug>::A) -> <Self as Plug<B>>::Out,
    {
        self.and_then(f)
    }
}

impl<A: Semigroup> Semigroup for Option<A> {
    fn combine(self, other: Option<A>) -> Option<A> {
        match (self, other) {
            (a, None) => a,
            (None, b) => b,
            (Some(a), Some(b)) => Some(a.combine(b)),
        }
    }
}

impl<A: Semigroup> Monoid for Option<A> {
    fn mempty() -> Self {
        None
    }
}

impl<A> Foldable for Option<A> {
    fn fold_left<B, F>(self, init: B, f: F) -> B
    where
        F: FnOnce(B, <Self as Unplug>::A) -> B,
    {
        match self {
            Some(a) => f(init, a),
            None => init,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::classes::*;

    impl Semigroup for i32 {
        fn combine(self, other: i32) -> i32 {
            self + other
        }
    }

    #[test]
    fn test_monoid() {
        assert_eq!(Option::<i32>::mempty(), None);
        assert_eq!(Semigroup::combine(1, 2), 3);
        assert_eq!(Option::mempty().combine(Some(1)).combine(Some(2)), Some(3));
    }

    #[test]
    fn test_functor_applicative_monad() {
        assert_eq!(Some(2).fmap(|a| a + 1), Some(3));
        assert_eq!(None.fmap(|a: i32| a + 1), None);

        assert_eq!(Option::pure(2), Some(2));
        assert_eq!(Some(2).ap(Some(|a| a + 1)), Some(3));
        assert_eq!(Some(2).ap::<i32, fn(i32) -> i32>(None), None);
        assert_eq!(None.ap(Some(|a: i32| a + 1)), None);

        assert_eq!(Some(2).bind(|a: i32| a.checked_add(1)), Some(3));
        assert_eq!(None.bind(|a: i32| a.checked_add(1)), None);
        assert_eq!(Some(2).bind(|_| None::<i32>), None);
    }
}
