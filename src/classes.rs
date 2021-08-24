use crate::mirror::{Hkt1, Hkt2, Mirror1, Mirror1Ext, Mirror2, Mirror2Ext};

pub trait Functor: Hkt1 {
    fn fmap<A, B, F: Fn(A) -> B>(f: F, fa: Self::Member<A>) -> Self::Member<B>;
}

pub trait FunctorExt: Mirror1 + Sized
where
    Self::Family: Functor,
{
    fn fmap<B, F: Fn(Self::T) -> B>(self, f: F) -> <Self::Family as Hkt1>::Member<B> {
        <Self::Family as Functor>::fmap(f, self.as_member())
    }
}

impl<F: Functor, T: Mirror1<Family = F>> FunctorExt for T {}

pub trait Apply: Functor {
    fn ap<A: Clone, B, F: Fn(A) -> B>(fa: Self::Member<F>, fb: Self::Member<A>) -> Self::Member<B>;
}

pub trait ApplyExt: Mirror1 + Sized
where
    Self::Family: Apply,
{
    fn ap<B, F: Fn(Self::T) -> B>(
        self,
        f: <Self::Family as Hkt1>::Member<F>,
    ) -> <Self::Family as Hkt1>::Member<B>
    where
        <Self as Mirror1>::T: Clone,
    {
        <Self::Family as Apply>::ap(f, self.as_member())
    }
}

impl<F: Apply, T: Mirror1<Family = F>> ApplyExt for T {}

pub trait Applicative: Apply {
    fn pure<A>(a: A) -> Self::Member<A>;
}

pub trait ApplicativeExt: Mirror1 + Sized
where
    Self::Family: Applicative,
{
    fn pure(a: Self::T) -> <Self::Family as Hkt1>::Member<Self::T> {
        <Self::Family as Applicative>::pure(a)
    }
}

impl<F: Applicative, T: Mirror1<Family = F>> ApplicativeExt for T {}

pub trait Monad: Applicative {
    fn bind<A, B, F: Fn(A) -> Self::Member<B>>(a: Self::Member<A>, f: F) -> Self::Member<B>;
}

pub trait MonadExt: Mirror1 + Sized
where
    Self::Family: Monad,
{
    fn bind<B, F: Fn(Self::T) -> <Self::Family as Hkt1>::Member<B>>(
        self,
        f: F,
    ) -> <Self::Family as Hkt1>::Member<B> {
        <Self::Family as Monad>::bind(self.as_member(), f)
    }
}

impl<F: Monad, T: Mirror1<Family = F>> MonadExt for T {}

pub trait Traversable: Hkt1 {
    fn traverse<App: Applicative, A, B, F: Fn(A) -> App::Member<B>>(
        f: F,
        t: Self::Member<A>,
    ) -> App::Member<Self::Member<B>>;
}

pub trait TraversableExt: Mirror1 + Sized
where
    Self::Family: Traversable,
{
    fn traverse<AppB: Mirror1, F: Fn(Self::T) -> AppB>(
        self,
        f: F,
    ) -> <AppB::Family as Hkt1>::Member<<Self::Family as Hkt1>::Member<AppB::T>>
    where
        AppB::Family: Applicative,
    {
        <Self::Family as Traversable>::traverse::<AppB::Family, _, _, _>(
            |t| f(t).as_member(),
            self.as_member(),
        )
    }
}

impl<F: Traversable, T: Mirror1<Family = F>> TraversableExt for T {}

pub trait Bifunctor: Hkt2 {
    fn bimap<A, B, C, D, F1: Fn(A) -> B, F2: Fn(C) -> D>(
        f1: F1,
        f2: F2,
        f: Self::Member<A, C>,
    ) -> Self::Member<B, D>;
}

pub trait BifunctorExt: Mirror2 + Sized
where
    Self::Family: Bifunctor,
{
    fn bimap<C, D, F1: Fn(Self::A) -> C, F2: Fn(Self::B) -> D>(
        self,
        f1: F1,
        f2: F2,
    ) -> <Self::Family as Hkt2>::Member<C, D> {
        <Self::Family as Bifunctor>::bimap(f1, f2, self.as_member())
    }
}

impl<F: Bifunctor, T: Mirror2<Family = F>> BifunctorExt for T {}

pub trait Semigroup {
    fn combine(self, other: Self) -> Self;
}

pub trait Monoid: Semigroup {
    fn mempty() -> Self;
}

pub trait Foldable: Hkt1 {
    fn fold_left<A, B, F: Fn(B, A) -> B>(f: F, init: B, t: Self::Member<A>) -> B;

    fn fold_right<A, B, F: Fn(A, B) -> B>(f: F, init: B, t: Self::Member<A>) -> B;
}

pub trait FoldableExt: Mirror1 + Sized
where
    Self::Family: Foldable,
{
    fn fold_left<B, F: Fn(B, Self::T) -> B>(self, init: B, f: F) -> B {
        <Self::Family as Foldable>::fold_left(f, init, self.as_member())
    }

    fn fold_right<B, F: Fn(Self::T, B) -> B>(self, init: B, f: F) -> B {
        <Self::Family as Foldable>::fold_right(f, init, self.as_member())
    }
}

impl<F: Foldable, T: Mirror1<Family = F>> FoldableExt for T {}

pub trait Show {
    fn show(a: Self) -> String;
}

pub trait Contravariant: Hkt1 {
    fn contramap<A, B, F: Fn(A) -> B>(f: F, fa: Self::Member<B>) -> Self::Member<A>;
}

pub trait ContravariantExt: Mirror1 + Sized
where
    Self::Family: Contravariant,
{
    fn contramap<A, F: Fn(A) -> Self::T>(self, f: F) -> <Self::Family as Hkt1>::Member<A> {
        <Self::Family as Contravariant>::contramap(f, self.as_member())
    }
}

impl<F: Contravariant, T: Mirror1<Family = F>> ContravariantExt for T {}

pub trait SemigroupK: Hkt1 {
    fn combine_k<A>(fa: Self::Member<A>, fb: Self::Member<A>) -> Self::Member<A>;
}

pub trait SemigroupKExt: Mirror1 + Sized
where
    Self::Family: SemigroupK,
{
    fn combine_k(
        self,
        other: <Self::Family as Hkt1>::Member<Self::T>,
    ) -> <Self::Family as Hkt1>::Member<Self::T> {
        <Self::Family as SemigroupK>::combine_k(self.as_member(), other)
    }
}

impl<F: SemigroupK, T: Mirror1<Family = F>> SemigroupKExt for T {}

pub trait MonoidK: SemigroupK {
    fn empty<A>() -> Self::Member<A>;
}
pub trait MonoidKExt: Mirror1 + Sized
where
    Self::Family: MonoidK,
{
    fn empty() -> <Self::Family as Hkt1>::Member<Self::T> {
        <Self::Family as MonoidK>::empty()
    }
}

impl<F: MonoidK, T: Mirror1<Family = F>> MonoidKExt for T {}

pub trait Alternative: Applicative + MonoidK {}
