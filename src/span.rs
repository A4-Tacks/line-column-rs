//! Out of the box [`Span`] for storing source code and text range.

use core::{fmt, ops};
use std::{string::String, sync::Arc};

pub use text_size::{TextRange, TextSize};

pub mod wrapper;

/// [`text_size::TextRange`] wrapper
///
/// Stored source code pointers, allowing for easy retrieval of lines, columns, and source code text
///
/// If `len() == 0`, it is used to indicate offset
///
/// # Examples
///
/// ```
/// use line_column::span::*;
///
/// let source = Span::new_full("foo,bar,baz");
/// let comma = source.create(TextRange::at(3.into(), TextSize::of(',')));
/// let bar = comma.after().take(TextSize::of("bar"));
///
/// assert_eq!(comma.text(), ",");
/// assert_eq!(bar.text(), "bar");
/// assert_eq!(bar.source(), "foo,bar,baz");
/// assert_eq!(bar.line_column(), (1, 5));
/// ```
#[derive(Clone, Default)]
pub struct Span {
    source: Arc<String>,
    range: TextRange,
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = self.text();
        write!(f, "Span({text:?}@{:?})", self.range())
    }
}

impl Span {
    /// New a source and span range.
    ///
    /// **NOTE**: It is not recommended to call repeatedly,
    /// otherwise the `source` will be allocated repeatedly.  Consider using [`Span::create`]
    ///
    /// # Panics
    ///
    /// - Panics if `range` out of source.
    /// - Panics if `source.len()` out of [`TextSize`].
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let source = "abcdef";
    /// let span = Span::new(source, TextRange::new(2.into(), 4.into()));
    /// assert_eq!(span.text(), "cd");
    /// ```
    #[inline]
    #[track_caller]
    pub fn new(source: impl Into<String>, range: TextRange) -> Self {
        Self::checked_new(source.into().into(), range)
    }

    /// New a full span of source.
    ///
    /// **NOTE**: It is not recommended to call repeatedly,
    /// otherwise the `source` will be allocated repeatedly.  Consider using [`Span::create`]
    ///
    /// # Panics
    ///
    /// - Panics if `source.len()` out of [`TextSize`].
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let source = "abcdef";
    /// let full = Span::new_full(source);
    /// assert_eq!(full.text(), "abcdef");
    /// ```
    #[inline]
    pub fn new_full(source: impl Into<String>) -> Self {
        let source = source.into();
        let range = TextRange::up_to(len_size(source.len()));
        Self::checked_new(source.into(), range)
    }

    /// New a span source range from exist span.
    ///
    /// # Panics
    ///
    /// - Panics if `range` out of source.
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let source = "abcdef";
    /// let full = Span::new_full(source);
    /// assert_eq!(full.text(), "abcdef");
    ///
    /// let span = full.create(TextRange::at(1.into(), 3.into()));
    /// assert_eq!(span.text(), "bcd");
    /// let span2 = span.create(TextRange::at(3.into(), 3.into()));
    /// assert_eq!(span2.text(), "def");
    /// ```
    #[inline]
    #[track_caller]
    pub fn create(&self, range: TextRange) -> Self {
        Self::checked_new(self.source.clone(), range)
    }

    /// New a span relative range from exist span.
    ///
    /// # Panics
    ///
    /// - Panics if `range+start` out of source.
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let source = "abcdef";
    /// let full = Span::new_full(source);
    /// assert_eq!(full.text(), "abcdef");
    ///
    /// let span = full.slice(TextRange::at(1.into(), 3.into()));
    /// assert_eq!(span.text(), "bcd");
    /// let span2 = span.slice(TextRange::at(1.into(), 3.into()));
    /// assert_eq!(span2.text(), "cde");
    /// ```
    #[inline]
    #[track_caller]
    pub fn slice(&self, range: TextRange) -> Self {
        let start = self.range.start();
        self.create(range+start)
    }

    /// New splited span pair relative range from exist span.
    ///
    /// # Panics
    ///
    /// - Panics if `range+at` out of source.
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let source = "abcdef";
    /// let full = Span::new_full(source);
    /// assert_eq!(full.text(), "abcdef");
    ///
    /// let (a, span) = full.split(TextSize::of("a"));
    /// assert_eq!(a.text(), "a");
    /// assert_eq!(span.text(), "bcdef");
    ///
    /// let (bcd, span2) = span.split(TextSize::of("bcd"));
    /// assert_eq!(bcd.text(), "bcd");
    /// assert_eq!(span2.text(), "ef");
    /// ```
    #[inline]
    #[track_caller]
    pub fn split(&self, len: TextSize) -> (Self, Self) {
        let start = self.range.start();
        let end = self.range.end();
        let point = start + len;
        (
            self.create(TextRange::new(start, point)),
            self.create(TextRange::new(point, end)),
        )
    }

    #[inline]
    #[track_caller]
    fn checked_new(source: Arc<String>, range: TextRange) -> Self {
        let source_length = len_size(source.len());

        assert!(range.end() <= source_length, "range end > source length ({:?} > {source_length:?})", range.end());

        Self { source, range }
    }

    /// Returns the is empty of this [`Span`] range.
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let span = Span::new_full("foo");
    /// let empty = span.create(TextRange::empty(1.into()));
    /// assert_eq!(span.is_empty(),  false);
    /// assert_eq!(empty.is_empty(), true);
    /// assert_eq!(empty.range(),    TextRange::new(1.into(), 1.into()));
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.range().is_empty()
    }

    /// Returns the length of this [`Span`] range.
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let span = Span::new_full("foo");
    /// let empty = span.create(TextRange::empty(1.into()));
    /// assert_eq!(span.len(),  TextSize::new(3));
    /// assert_eq!(empty.len(), TextSize::new(0));
    /// ```
    #[inline]
    pub fn len(&self) -> TextSize {
        self.range().len()
    }

    /// Returns the source before of this [`Span`].
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let span = Span::new("foobarbaz", TextRange::new(3.into(), 6.into()));
    /// assert_eq!(span.text(),          "bar");
    /// assert_eq!(span.before().text(), "foo");
    /// ```
    pub fn before(&self) -> Self {
        let range = TextRange::up_to(self.range().start());
        self.create(range)
    }

    /// Returns the source after of this [`Span`].
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let span = Span::new("foobarbaz", TextRange::new(3.into(), 6.into()));
    /// assert_eq!(span.text(),          "bar");
    /// assert_eq!(span.after().text(),  "baz");
    /// ```
    pub fn after(&self) -> Self {
        let end = TextSize::of(self.source());
        let range = TextRange::new(self.range().end(), end);
        self.create(range)
    }

    /// Returns truncated sub-span.
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let span = Span::new("foobarbaz", TextRange::new(3.into(), 7.into()));
    /// assert_eq!(span.text(), "barb");
    /// assert_eq!(span.take(3.into()).text(), "bar");
    /// ```
    pub fn take(&self, len: TextSize) -> Self {
        let range = self.range;
        let new_len = range.len().min(len);
        let new_range = TextRange::at(self.range.start(), new_len);
        self.create(new_range)
    }

    /// Returns the start of this [`Span`].
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let span = Span::new("abcdef", TextRange::new(1.into(), 4.into()));
    /// assert_eq!(span.start().range(), TextRange::new(1.into(), 1.into()));
    /// ```
    pub fn start(&self) -> Self {
        self.create(TextRange::empty(self.range.start()))
    }

    /// Returns the end of this [`Span`].
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let span = Span::new("abcdef", TextRange::new(1.into(), 4.into()));
    /// assert_eq!(span.end().range(), TextRange::new(4.into(), 4.into()));
    /// ```
    pub fn end(&self) -> Self {
        self.create(TextRange::empty(self.range.end()))
    }

    /// Returns the start index of this [`Span`] range.
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let span = Span::new("abcdef", TextRange::new(1.into(), 4.into()));
    /// assert_eq!(span.index(), TextSize::new(1));
    /// ```
    #[inline]
    pub fn index(&self) -> TextSize {
        self.range().start()
    }

    /// Returns the source text of the range reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let span = Span::new("abcdef", TextRange::new(1.into(), 4.into()));
    /// assert_eq!(span.text(), "bcd");
    /// ```
    #[doc(alias = "as_str")]
    pub fn text(&self) -> &str {
        &self.source()[self.range()]
    }

    /// Returns the source text of the range reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let span = Span::new("abcdef", TextRange::new(1.into(), 4.into()));
    /// assert_eq!(span.range(),       TextRange::new(1.into(), 4.into()));
    /// ```
    pub fn range(&self) -> TextRange {
        self.range
    }

    /// Returns the source text.
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let span = Span::new("abcdef", TextRange::new(1.into(), 4.into()));
    /// assert_eq!(span.source(), "abcdef");
    /// assert_eq!(span.text(),   "bcd");
    /// ```
    pub fn source(&self) -> &str {
        &self.source
    }
}

impl Span {
    pub fn line_column(&self) -> (u32, u32) {
        crate::line_column(self.source(), self.index().into())
    }

    pub fn line(&self) -> u32 {
        self.line_column().0
    }

    pub fn column(&self) -> u32 {
        self.line_column().1
    }

    /// Returns the current line of this [`Span`].
    ///
    /// Maybe include end of line char, like `'\n'`.
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let span = Span::new_full("foo\nbar\nbaz");
    /// let next = span.create(TextRange::at(TextSize::of("foo\n"), 5.into()));
    /// let tail = span.create(TextRange::at(TextSize::of("foo\nbar\n"), 3.into()));
    /// let endl = span.create(TextRange::at(TextSize::of("foo"), 3.into()));
    ///
    /// assert_eq!(next.text(), "bar\nb");
    /// assert_eq!(tail.text(), "baz");
    /// assert_eq!(endl.text(), "\nba");
    ///
    /// assert_eq!(span.current_line().text(), "foo\n");
    /// assert_eq!(next.current_line().text(), "bar\n");
    /// assert_eq!(tail.current_line().text(), "baz");
    /// assert_eq!(endl.current_line().text(), "foo\n");
    /// ```
    pub fn current_line(&self) -> Self {
        let before = &self.source[..self.range.start().into()];
        let line_start = before.rfind('\n').map_or(0, |it| it+1);
        let rest = &self.source[line_start..];

        let line_len = match rest.split_once('\n') {
            Some((line, _)) => TextSize::of(line) + TextSize::of('\n'),
            None => TextSize::of(rest),
        };
        let range = TextRange::at(len_size(line_start), line_len);
        self.create(range)
    }

    /// Returns the previous line of this [`Span`].
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let span = Span::new_full("foo\nbar\nbaz");
    /// let next = span.create(TextRange::at(TextSize::of("foo\n"), 5.into()));
    /// let tail = span.create(TextRange::at(TextSize::of("foo\nbar\n"), 3.into()));
    /// let endl = span.create(TextRange::at(TextSize::of("foo"), 3.into()));
    ///
    /// assert_eq!(next.text(), "bar\nb");
    /// assert_eq!(tail.text(), "baz");
    /// assert_eq!(endl.text(), "\nba");
    ///
    /// assert_eq!(span.prev_line().text(), "");
    /// assert_eq!(next.prev_line().text(), "foo\n");
    /// assert_eq!(tail.prev_line().text(), "bar\n");
    /// assert_eq!(endl.prev_line().text(), "");
    /// ```
    pub fn prev_line(&self) -> Self {
        let index = self.current_line().index();
        if let Some(prev_line_offset) = index.checked_sub(TextSize::of('\n')) {
            self.create(TextRange::empty(prev_line_offset)).current_line()
        } else {
            self.create(TextRange::empty(TextSize::new(0)))
        }
    }

    /// Returns the next line of this [`Span`].
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let span = Span::new_full("foo\nbar\nbaz");
    /// let next = span.create(TextRange::at(TextSize::of("foo\n"), 5.into()));
    /// let tail = span.create(TextRange::at(TextSize::of("foo\nbar\n"), 3.into()));
    /// let endl = span.create(TextRange::at(TextSize::of("foo"), 3.into()));
    ///
    /// assert_eq!(next.text(), "bar\nb");
    /// assert_eq!(tail.text(), "baz");
    /// assert_eq!(endl.text(), "\nba");
    ///
    /// assert_eq!(span.next_line().text(), "bar\n");
    /// assert_eq!(next.next_line().text(), "baz");
    /// assert_eq!(tail.next_line().text(), "");
    /// assert_eq!(endl.next_line().text(), "bar\n");
    /// ```
    pub fn next_line(&self) -> Self {
        let cur_line_end = self.current_line().range().end();
        if self.source().len() == cur_line_end.into() {
            self.create(TextRange::empty(cur_line_end))
        } else {
            let range = TextRange::empty(cur_line_end);
            self.create(range).current_line()
        }
    }
}

impl Span {
    /// Returns the trim end of this [`Span`] range.
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let span = Span::new("foo  bar  baz", TextRange::new(4.into(), 9.into()));
    /// assert_eq!(span.text(), " bar ");
    /// assert_eq!(span.trim_end().text(), " bar");
    /// ```
    pub fn trim_end(&self) -> Self {
        let text = self.text();
        let trimmed = text.trim_end();
        let len = TextSize::of(trimmed);
        self.create(TextRange::at(self.range.start(), len))
    }

    /// Returns the trim start of this [`Span`] range.
    ///
    /// # Examples
    ///
    /// ```
    /// use line_column::span::*;
    ///
    /// let span = Span::new("foo  bar  baz", TextRange::new(4.into(), 9.into()));
    /// assert_eq!(span.text(), " bar ");
    /// assert_eq!(span.trim_start().text(), "bar ");
    /// ```
    pub fn trim_start(&self) -> Self {
        let text = self.text();
        let trimmed = text.trim_start();
        let len = TextSize::of(trimmed);

        let offset = TextSize::of(text) - len;
        let start = self.range.start() + offset;
        self.create(TextRange::at(start, len))
    }
}

#[inline]
#[track_caller]
fn len_size(len: usize) -> TextSize {
    match TextSize::try_from(len) {
        Ok(source_length) => source_length,
        _ => panic!("source length {len} overflow TextSize"),
    }
}

#[cfg(test)]
mod tests {
    use core::iter::successors;
    use std::{format, vec::Vec};

    use super::*;

    #[track_caller]
    fn check_texts(spans: impl IntoIterator<Item = Span>, expect: &[&str]) {
        let spans = Vec::from_iter(spans);
        let texts = spans.iter().map(|it| it.text()).collect::<Vec<_>>();
        assert_eq!(texts, expect);
    }

    #[test]
    #[should_panic = "range end > source length"]
    fn new_panic_out_of_source() {
        let _span = Span::new("x", TextRange::up_to(TextSize::of("xy")));
    }

    #[test]
    fn next_lines_without_end_eol() {
        let source = "foo\nbar\n\nbaz";
        let span = Span::new_full(source);
        let lines =
            successors(span.current_line().into(), |it| Some(it.next_line()))
                .take_while(|it| !it.is_empty())
                .collect::<Vec<_>>();
        check_texts(lines, &[
            "foo\n",
            "bar\n",
            "\n",
            "baz",
        ]);
    }

    #[test]
    fn next_lines_multi_bytes_char() {
        let source = "测试\n实现\n\n多字节";
        let span = Span::new_full(source);
        let lines =
            successors(span.current_line().into(), |it| Some(it.next_line()))
                .take_while(|it| !it.is_empty())
                .collect::<Vec<_>>();
        check_texts(lines, &[
            "测试\n",
            "实现\n",
            "\n",
            "多字节",
        ]);
    }

    #[test]
    fn next_lines_with_end_eol() {
        let source = "foo\nbar\n\nbaz\n";
        let span = Span::new_full(source);
        let lines =
            successors(span.current_line().into(), |it| Some(it.next_line()))
                .take_while(|it| !it.is_empty())
                .collect::<Vec<_>>();
        check_texts(lines, &[
            "foo\n",
            "bar\n",
            "\n",
            "baz\n",
        ]);
    }

    #[test]
    fn next_lines_first_empty_line() {
        let source = "\nfoo\nbar\n\nbaz";
        let span = Span::new_full(source);
        let lines =
            successors(span.current_line().into(), |it| Some(it.next_line()))
                .take_while(|it| !it.is_empty())
                .collect::<Vec<_>>();
        check_texts(lines, &[
            "\n",
            "foo\n",
            "bar\n",
            "\n",
            "baz",
        ]);
    }

    #[test]
    fn prev_lines_with_end_eol() {
        let source = "foo\nbar\n\nbaz\n";
        let span = Span::new(source, TextRange::empty(TextSize::of(source)));
        let lines =
            successors(span.current_line().into(), |it| Some(it.prev_line()))
                .skip(1)
                .take_while(|it| !it.is_empty())
                .collect::<Vec<_>>();
        check_texts(lines, &[
            "baz\n",
            "\n",
            "bar\n",
            "foo\n",
        ]);
    }

    #[test]
    fn prev_lines_without_end_eol() {
        let source = "foo\nbar\n\nbaz";
        let span = Span::new(source, TextRange::empty(TextSize::of(source)));
        let lines =
            successors(span.current_line().into(), |it| Some(it.prev_line()))
                .take_while(|it| !it.is_empty())
                .collect::<Vec<_>>();
        check_texts(lines, &[
            "baz",
            "\n",
            "bar\n",
            "foo\n",
        ]);
    }

    #[test]
    fn prev_lines_multi_bytes_char() {
        let source = "测试\n实现\n\n多字节";
        let span = Span::new(source, TextRange::empty(TextSize::of(source)));
        let lines =
            successors(span.current_line().into(), |it| Some(it.prev_line()))
                .take_while(|it| !it.is_empty())
                .collect::<Vec<_>>();
        check_texts(lines, &[
            "多字节",
            "\n",
            "实现\n",
            "测试\n",
        ]);
    }

    #[test]
    fn test_trim_start() {
        let datas = [
            "",
            "f",
            "foo",
            " ",
            " f",
            " foo",
            "  ",
            "  f",
            "  foo",
            "  f",
            "  foo",
        ];
        for prefix in ["", "x"] {
            for suffix in ["", "x", " ", "  "] {
                for data in datas {
                    let source = format!("{prefix}{data}{suffix}");
                    let range = TextRange::new(
                        TextSize::of(prefix),
                        TextSize::of(&source),
                    );
                    let span = Span::new(&source, range);
                    assert_eq!(span.trim_start().text(), source[range].trim_start());
                }
            }
        }
    }

    #[test]
    fn test_trim_end() {
        let datas = [
            "",
            "f",
            "foo",
            " ",
            " f",
            "foo ",
            "  ",
            "f  ",
            "foo  ",
            "f  ",
            "foo  ",
        ];
        for prefix in ["", "x", " ", "  "] {
            for suffix in ["", "x"] {
                for data in datas {
                    let source = format!("{prefix}{data}{suffix}");
                    let range = TextRange::new(
                        TextSize::new(0),
                        TextSize::of(&source) - TextSize::of(suffix),
                    );
                    let span = Span::new(&source, range);
                    assert_eq!(span.trim_end().text(), source[range].trim_end());
                }
            }
        }
    }
}
