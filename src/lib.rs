mod tests;

pub fn bluebird<A, B, C, X, Y>(x: X, y: Y, z: A) -> C
where
    X: Fn(B) -> C,
    Y: Fn(A) -> B,
{
    x(y(z))
}

pub fn blackbird<A, B, C, D, X, Y>(x: X, y: Y, z: A, w: B) -> D
where
    X: Fn(C) -> D,
    Y: Fn(A, B) -> C,
{
    x(y(z, w))
}
