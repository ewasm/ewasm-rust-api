//! Basic type conversion traits. Unlike the native standard library, `U: From<T>` does not yet
//! imply `T: Into<U>`.

pub trait From<T>: Sized {
    fn from(_: T) -> Self;
}

pub trait Into<T>: Sized {
    fn from(self) -> T;
}
