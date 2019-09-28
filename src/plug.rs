/// Unplug destructures F<A> into F and A.
pub trait Unplug {
    type F;
    type A;
}

/// Unplug2 destructures F<A, B> into F, A, and B.
pub trait Unplug2 {
    type F;
    type A;
    type B;
}

/// Plug replaces the type F<B> to F<A>.
pub trait Plug<A> {
    type Out: Unplug<A = A>;
}

/// Plug2 replaces the type F<C, D> to F<A, B>.
pub trait Plug2<A, B> {
    type Out: Unplug2<A = A, B = B>;
}