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

#[cfg(test)]
mod tests {
    use crate::classes::*;

    #[test]
    fn test_functor() {
        assert_eq!(vec![1, 2, 3].fmap(|a| a + 1), vec![2, 3, 4]);
        assert_eq!(vec![].fmap(|a: i32| a + 1), vec![]);
    }

    #[test]
    fn test_applicative() {
        let plus_one: Box<dyn Fn(i32) -> i32> = Box::new(|a| a + 1);
        let plus_two: Box<dyn Fn(i32) -> i32> = Box::new(|a| a + 2);
        let plus_three: Box<dyn Fn(i32) -> i32> = Box::new(|a| a + 3);
        assert_eq!(
            vec![1, 2, 3].ap(vec![plus_one, plus_two, plus_three]),
            vec![2, 3, 4, 3, 4, 5, 4, 5, 6]
        );
    }

    #[test]
    fn test_monad() {
        assert_eq!(
            vec![1, 2, 3].bind(|i| vec![2, 3, 4].bind(|j| vec![(i, j)])),
            vec![
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 2),
                (2, 3),
                (2, 4),
                (3, 2),
                (3, 3),
                (3, 4)
            ]
        );
    }

    #[test]
    fn test_monoid() {
        assert_eq!(vec![1, 2, 3].combine(vec![4, 5, 6]), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(Vec::<i32>::mempty(), vec![]);
        assert_eq!(Vec::mempty().combine(vec![1]).combine(vec![2]), vec![1, 2]);
    }

    #[test]
    fn test_foldable() {
        assert_eq!(
            vec![1, 2, 3, 4, 5].fold_left("".to_owned(), |acc, i| format!("{}{}", acc, i)),
            "12345"
        );
        assert_eq!(
            vec![1, 2, 3, 4, 5].fold_right("".to_owned(), |i, acc| format!("{}{}", acc, i)),
            "54321"
        );
    }

    #[test]
    fn test_alternative() {
        assert_eq!(vec![1, 2, 3].combine(vec![4, 5, 6]), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(Vec::<i32>::empty(), vec![]);
    }
}
