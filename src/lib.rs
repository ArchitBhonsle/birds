mod tests;

/// B combinator: Bxyz = x(yz)
pub fn bluebird<A, B, C, X, Y>(x: X, y: Y, z: A) -> C
where
    X: Fn(B) -> C,
    Y: Fn(A) -> B,
{
    x(y(z))
}

/// C combinator: Cxyz = xzy
pub fn cardinal<C, X, Y, Z>(x: X, y: Y, z: Z) -> C
where
    X: Fn(Z, Y) -> C,
{
    x(z, y)
}
