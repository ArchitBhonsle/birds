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

/// G combinator: Gxyzw = xw(yz)
pub fn goldfinch<A, B, X, Y, Z, W>(x: X, y: Y, z: Z, w: W) -> A
where
    X: Fn(W, B) -> A,
    Y: Fn(Z) -> B,
{
    x(w, y(z))
}

/// H combinator: Hxyz = xyzy
pub fn hummingbird<A, X, Y, Z>(x: X, y: Y, z: Z) -> A
where
    X: Fn(Y, Z, Y) -> A,
    Y: Clone, // TODO taking references at parameters seems better
{
    x(y.clone(), z, y)
}

/// I combinator: Ix = x
pub fn identity_bird<X>(x: X) -> X {
    x
}

/// J combinator: Jxyzw = xy(xwz)
pub fn jay<X, Y, Z>(x: X, y: Y, z: Z, w: Y) -> Z
where
    X: Fn(Y, Z) -> Z,
{
    x(y, x(w, z))
}

/// K combinator: Kxy = x
pub fn kestrel<X, Y>(x: X, _y: Y) -> X {
    x
}

/// L combinator: Lxy = Lx(yy)
pub fn lark<A, B, X, Y>(x: X, y: Y) -> A
where
    X: Fn(B) -> A,
    Y: Fn(Y) -> B + Clone,
{
    x(y(y.clone()))
}

/// M combinator: Mx = xx
pub fn mockingbird<A, X>(x: X) -> A
where
    X: Fn(X) -> A + Clone,
{
    x(x.clone())
}
