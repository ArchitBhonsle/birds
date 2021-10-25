mod tests;

#[macro_export]
macro_rules! idiot {
    ($a:ident) => {
        $a
    };
}

#[macro_export]
macro_rules! starling {
    ($a:ident $b:ident $c:ident) => {
        $a($b)($b($c))
    };
}

#[macro_export]
macro_rules! kestrel {
    ($a:ident $b:ident) => {
        $a
    };
}
