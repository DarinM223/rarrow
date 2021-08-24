use crate::classes::*;
use crate::mirror::{Hkt1, Mirror1};
use std::fmt::Debug;

impl<A> Mirror1 for Option<A> {
    type T = A;
    type Family = OptionFamily;
}

pub struct OptionFamily;
impl Hkt1 for OptionFamily {
    type Member<T> = Option<T>;
}

impl Functor for OptionFamily {
    fn fmap<A, B, F: Fn(A) -> B>(f: F, fa: Option<A>) -> Option<B> {
        fa.map(f)
    }
}

impl Apply for OptionFamily {
    fn ap<A, B, F: Fn(A) -> B>(fa: Option<F>, fb: Option<A>) -> Option<B> {
        fa.and_then(|fun| fb.map(fun))
    }
}

impl Applicative for OptionFamily {
    fn pure<A>(a: A) -> Option<A> {
        Some(a)
    }
}

impl Monad for OptionFamily {
    fn bind<A, B, F: Fn(A) -> Option<B>>(fa: Option<A>, f: F) -> Option<B> {
        fa.and_then(f)
    }
}

impl Traversable for OptionFamily {
    fn traverse<App: Applicative, A, B, F: Fn(A) -> App::Member<B>>(
        f: F,
        t: Option<A>,
    ) -> App::Member<Option<B>> {
        match t {
            Some(v) => App::fmap(|v| Some(v), f(v)),
            None => App::pure(None),
        }
    }
}

impl Foldable for OptionFamily {
    fn fold_left<A, B, F: Fn(B, A) -> B>(f: F, init: B, t: Option<A>) -> B {
        match t {
            Some(a) => f(init, a),
            None => init,
        }
    }

    fn fold_right<A, B, F: Fn(A, B) -> B>(f: F, init: B, t: Option<A>) -> B {
        OptionFamily::fold_left(|b, a| f(a, b), init, t)
    }
}

impl SemigroupK for OptionFamily {
    fn combine_k<A>(fa: Option<A>, fb: Option<A>) -> Option<A> {
        match (fa, fb) {
            (None, b) => b,
            (a, _) => a,
        }
    }
}

impl MonoidK for OptionFamily {
    fn empty<A>() -> Option<A> {
        None
    }
}

impl Alternative for OptionFamily {}

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

impl<A: Debug> Show for Option<A> {
    fn show(a: Option<A>) -> String {
        format!("{:?}", a)
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
        let v: Option<i32> = Option::pure(2);
        assert_eq!(v, Some(2));
        assert_eq!(Some(2).ap(Some(|a| a + 1)), Some(3));
        assert_eq!(Some(2).ap::<i32, fn(i32) -> i32>(None), None);
        assert_eq!(None.ap(Some(|a: i32| a + 1)), None);

        assert_eq!(Some(2).bind(|a: i32| a.checked_add(1)), Some(3));
        assert_eq!(None.bind(|a: i32| a.checked_add(1)), None);
        assert_eq!(Some(2).bind(|_| None::<i32>), None);
    }

    #[test]
    fn test_fold() {
        assert_eq!(
            Some(2).fold_left("a".to_owned(), |acc, n| format!("{}{}", acc, n)),
            "a2".to_owned()
        );
        assert_eq!(
            Some(2).fold_right("a".to_owned(), |n, acc| format!("{}{}", acc, n)),
            "a2".to_owned()
        );
    }

    #[test]
    fn test_alternative() {
        assert_eq!(Option::<i32>::empty(), None);
        assert_eq!(Some(2).combine_k(Some(3)), Some(2));
        assert_eq!(None.combine_k(Some(3)), Some(3));
        assert_eq!(Some(2).combine_k(None), Some(2));
        assert_eq!(None::<i32>.combine_k(None), None);
    }
}
