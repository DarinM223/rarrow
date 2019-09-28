/// Unplug destructures F<A> into F and A.
pub trait Unplug {
    type F;
    type A;
}

/// Plug replaces the type F<B> to F<A>.
pub trait Plug<A> {
    type Out: Unplug<A = A>;
}