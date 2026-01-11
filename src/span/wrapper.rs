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

// FIXME
// 可以做一下 LikeRange, LikeText, LikeStrict, LikePhantom 去让 Ord, Hash, Borrow 只走固定的部分
// 记得支持 Borrow
