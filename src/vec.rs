use crate::classes::*;
use crate::mirror::{Hkt1, Mirror1};
use std::fmt::Debug;

impl<A> Mirror1 for Vec<A> {
    type T = A;
    type Family = VecFamily;
}

pub struct VecFamily;
impl Hkt1 for VecFamily {
    type Member<T> = Vec<T>;
}

impl Functor for VecFamily {
    fn fmap<A, B, F: Fn(A) -> B>(f: F, fa: Vec<A>) -> Vec<B> {
        fa.into_iter().map(f).collect()
    }
}

impl Apply for VecFamily {
    fn ap<A: Clone, B, F: Fn(A) -> B>(fa: Vec<F>, fb: Vec<A>) -> Vec<B> {
        let mut result = Vec::with_capacity(fa.len() * fb.len());
        for f in fa.into_iter() {
            for x in fb.iter() {
                result.push(f(x.clone()));
            }
        }
        result
    }
}

impl Applicative for VecFamily {
    fn pure<A>(a: A) -> Vec<A> {
        vec![a]
    }
}

impl Monad for VecFamily {
    fn bind<A, B, F: Fn(A) -> Vec<B>>(fa: Vec<A>, f: F) -> Vec<B> {
        fa.into_iter().flat_map(f).collect()
    }
}

impl Foldable for VecFamily {
    fn fold_left<A, B, F: Fn(B, A) -> B>(f: F, init: B, t: Vec<A>) -> B {
        t.into_iter().fold(init, f)
    }

    fn fold_right<A, B, F: Fn(A, B) -> B>(f: F, init: B, t: Vec<A>) -> B {
        t.into_iter().rev().fold(init, |a, b| f(b, a))
    }
}

impl Traversable for VecFamily {
    fn traverse<App: Applicative, A, B: Clone, F: Fn(A) -> App::Member<B>>(
        f: F,
        t: Vec<A>,
    ) -> App::Member<Vec<B>> {
        t.fold_left(App::pure(Vec::new()), |ys, x| {
            App::ap(
                App::fmap(
                    |e: B| {
                        move |mut v: Vec<B>| {
                            v.push(e.clone());
                            v
                        }
                    },
                    f(x),
                ),
                ys,
            )
        })
    }
}

impl SemigroupK for VecFamily {
    fn combine_k<A>(fa: Vec<A>, fb: Vec<A>) -> Vec<A> {
        fa.into_iter().chain(fb.into_iter()).collect()
    }
}

impl MonoidK for VecFamily {
    fn empty<A>() -> Vec<A> {
        vec![]
    }
}

impl Alternative for VecFamily {}

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

impl<A: Debug> Show for Vec<A> {
    fn show(a: Vec<A>) -> String {
        format!("{:?}", a)
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
    fn test_traverse() {
        assert_eq!(vec![1, 2, 3].traverse(|i| Some(i)), Some(vec![1, 2, 3]));
        assert_eq!(vec![].traverse(|i: i32| Some(i)), Some(vec![]));
        assert_eq!(vec![1, 2, 3].traverse(|_| None::<i32>), None);
        assert_eq!(
            vec![2, 4, 6, 8].traverse(|i| if i % 2 == 0 { vec![i] } else { vec![] }),
            vec![vec![2, 4, 6, 8]]
        );
        assert_eq!(
            vec![2, 4, 6, 7, 8].traverse(|i| if i % 2 == 0 { vec![i] } else { vec![] }),
            Vec::<Vec<i32>>::new()
        );
    }

    #[test]
    fn test_alternative() {
        assert_eq!(vec![1, 2, 3].combine(vec![4, 5, 6]), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(Vec::<i32>::empty(), vec![]);
    }
}
