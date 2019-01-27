use super::*;

pub trait From<T>: Sized {
    fn from(_: T) -> Self;
}

pub trait Into<T>: Sized {
    fn from(self) -> T;
}
