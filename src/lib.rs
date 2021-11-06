/// B combinator: Bxyz = x(yz)
pub fn bluebird<A, B, C, X, Y>(x: X, y: Y, z: C) -> A
where
    X: Fn(B) -> A,
    Y: Fn(C) -> B,
{
    x(y(z))
}

/// C combinator: Cxyz = xzy
pub fn cardinal<A, X, Y, Z>(x: X, y: Y, z: Z) -> A
where
    X: Fn(Z, Y) -> A,
{
    x(z, y)
}

/// D combinator: Dxyzw = xy(zw)
pub fn dove<A, B, X, Y, Z, W>(x: X, y: Y, z: Z, w: W) -> B
where
    X: Fn(Y, A) -> B,
    Z: Fn(W) -> A,
{
    x(y, z(w))
}

/// E combinator: Exyzwv = xy(zwv)
pub fn eagle<A, B, X, Y, Z, W, V>(x: X, y: Y, z: Z, w: W, v: V) -> A
where
    X: Fn(Y, B) -> A,
    Z: Fn(W, V) -> B,
{
    x(y, z(w, v))
}

/// F combinator: Fxyz = zyx
pub fn finch<A, X, Y, Z>(x: X, y: Y, z: Z) -> A
where
    Z: Fn(Y, X) -> A,
{
    z(y, x)
}
