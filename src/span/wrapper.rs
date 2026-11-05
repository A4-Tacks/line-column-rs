//! Some wrapper, such as [`LikeRange`], can be used for some hash map, sets.

use core::{borrow::Borrow, hash::Hash};

use super::*;

/// Newtype of `Span::len() == 0`
#[derive(Clone, Default)]
#[repr(transparent)]
pub struct EmptySpan {
    span: Span,
}
impl fmt::Debug for EmptySpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_assert!(self.is_empty());
        write!(f, "EmptySpan({:?})", self.range().start())
    }
}
impl ops::Deref for EmptySpan {
    type Target = Span;

    fn deref(&self) -> &Self::Target {
        &self.span
    }
}
impl From<EmptySpan> for Span {
    fn from(span: EmptySpan) -> Self {
        span.span
    }
}
impl From<&EmptySpan> for Span {
    fn from(span: &EmptySpan) -> Self {
        span.span.clone()
    }
}

/// data_wrapper! {}
macro_rules! data_wrapper {
    {
        #[$doc:meta]
        struct $name:ident$(($ty:ty))?[$($t:tt)*];
    } => {
        #[$doc]
        #[derive(Debug, Clone, Default)]
        pub struct $name(pub Span);

        impl Eq for $name {}
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                (&self.0.$($t)*).eq(&other.0.$($t)*)
            }

            #[allow(clippy::partialeq_ne_impl)]
            fn ne(&self, other: &Self) -> bool {
                (&self.0.$($t)*).ne(&other.0.$($t)*)
            }
        }
        impl Hash for $name {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.0.$($t)*.hash(state);
            }
        }
        data_wrapper!(@data $name$(($ty))?[$($t)*]);
    };
    (@data $name:ident[$($t:tt)*]) => {};
    (@data $name:ident($ty:ty)[$($t:tt)*]) => {
        impl PartialEq<$ty> for $name {
            fn eq(&self, other: &$ty) -> bool {
                self.0.$($t)*.eq(other)
            }

            #[allow(clippy::partialeq_ne_impl)]
            fn ne(&self, other: &$ty) -> bool {
                self.0.$($t)*.ne(other)
            }
        }
        impl Borrow<$ty> for $name {
            fn borrow(&self) -> &$ty {
                &self.0.$($t)*
            }
        }
        impl AsRef<$ty> for $name {
            fn as_ref(&self) -> &$ty {
                &self.0.$($t)*
            }
        }
    };
}

data_wrapper! {
    /// Wrapper of [`Span::range`], [`PartialEq`] and [`Hash`] use it.
    struct LikeRange(TextRange)[range];
}

data_wrapper! {
    /// Wrapper of [`Span::text`], [`PartialEq`] and [`Hash`] use it.
    struct LikeText(str)[text()];
}

data_wrapper! {
    /// Wrapper of ([`Span::range`], [`Span::text`]), [`PartialEq`] and [`Hash`] use it.
    struct LikeAbsolute[absolute_helper()];
}

impl Span {
    pub(self) fn absolute_helper(&self) -> (TextRange, &str) {
        (self.range(), self.text())
    }
}

// NOTE: 由于 TextRange 没实现 Ord, 所以我也不实现了

#[test]
fn like_range() {
    let a = Span::new_full("foo");
    let b = Span::new_full("bar");
    assert_eq!(LikeRange(a), LikeRange(b));

    let (a, b) = Span::new_full("foofoo")
        .split_at(TextSize::of("foo"));
    assert_ne!(LikeRange(a), LikeRange(b));

    let like_range = LikeRange(Span::new_full("ab"));
    let borrowed: &TextRange = like_range.borrow();
    assert_eq!(borrowed, &TextRange::up_to(TextSize::of("ab")))
}

#[test]
fn like_text() {
    let (a, b) = Span::new_full("foofoo")
        .split_at(TextSize::of("foo"));
    assert_eq!(LikeText(a), LikeText(b));

    let a = Span::new_full("foo");
    let b = Span::new_full("bar");
    assert_ne!(LikeText(a), LikeText(b));

    let like_text = LikeText(Span::new_full("ab"));
    let borrowed: &str = like_text.borrow();
    assert_eq!(borrowed, "ab")
}

#[test]
fn like_absolute() {
    let (a, _) = Span::new_full("foobar")
        .split_at(TextSize::of("foo"));
    let (b, _) = Span::new_full("foobaz")
        .split_at(TextSize::of("foo"));
    assert_eq!(LikeAbsolute(a), LikeAbsolute(b));

    let a = Span::new_full("foo");
    let b = Span::new_full("bar");
    assert_ne!(LikeAbsolute(a), LikeAbsolute(b));

    let (a, b) = Span::new_full("foofoo")
        .split_at(TextSize::of("foo"));
    assert_ne!(LikeAbsolute(a), LikeAbsolute(b));
}
