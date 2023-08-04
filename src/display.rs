use core::fmt::Display;

#[doc(hidden)]
pub trait AsDisplay<'a> {
    // TODO: convert to generic associated type.
    // https://github.com/dtolnay/thiserror/pull/253
    type Target: Display;

    fn as_display(&'a self) -> Self::Target;
}

impl<'a, T> AsDisplay<'a> for &T
where
    T: Display + 'a,
{
    type Target = &'a T;

    fn as_display(&'a self) -> Self::Target {
        *self
    }
}

#[cfg(feature = "std")]
impl<'a> AsDisplay<'a> for std::path::Path {
    type Target = std::path::Display<'a>;

    #[inline]
    fn as_display(&'a self) -> Self::Target {
        self.display()
    }
}

#[cfg(feature = "std")]
impl<'a> AsDisplay<'a> for std::path::PathBuf {
    type Target = std::path::Display<'a>;

    #[inline]
    fn as_display(&'a self) -> Self::Target {
        self.display()
    }
}
