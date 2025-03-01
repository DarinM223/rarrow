//! Mirror method of simulating higher kinded types.
//! This method works for Traversable but requires GATs
//! and is more verbose and difficult to use.

pub trait Hkt1 {
    type Member<T>: Mirror1<T = T, Family = Self>;
}

pub trait Mirror1 {
    type T;
    type Family: Hkt1Accepting<Self::T, GetMember = Self>;
}

pub trait Mirror1Ext: Mirror1 {
    fn as_member(self) -> <Self::Family as Hkt1>::Member<Self::T>
    where
        Self: Sized,
    {
        fn identity<F: Hkt1, T>(x: <F as Hkt1Accepting<T>>::GetMember) -> F::Member<T> {
            x
        }
        identity::<Self::Family, _>(self)
    }
}

impl<T: Mirror1 + ?Sized> Mirror1Ext for T {}

pub trait Hkt1Accepting<T>: Hkt1 {
    type GetMember;
}

impl<F: Hkt1 + ?Sized, T> Hkt1Accepting<T> for F {
    type GetMember = Self::Member<T>;
}

pub trait Hkt2 {
    type Member<A, B>: Mirror2<A = A, B = B, Family = Self>;
}

pub trait Mirror2 {
    type A;
    type B;
    type Family: Hkt2Accepting<Self::A, Self::B, GetMember = Self>;
}

pub trait Mirror2Ext: Mirror2 {
    fn as_member(self) -> <Self::Family as Hkt2>::Member<Self::A, Self::B>
    where
        Self: Sized,
    {
        fn identity<F: Hkt2, A, B>(x: <F as Hkt2Accepting<A, B>>::GetMember) -> F::Member<A, B> {
            x
        }
        identity::<Self::Family, _, _>(self)
    }
}

impl<T: Mirror2 + ?Sized> Mirror2Ext for T {}

pub trait Hkt2Accepting<A, B>: Hkt2 {
    type GetMember;
}

impl<F: Hkt2 + ?Sized, A, B> Hkt2Accepting<A, B> for F {
    type GetMember = Self::Member<A, B>;
}
