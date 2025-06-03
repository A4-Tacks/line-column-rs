#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(test)]
mod tests;

const UNINIT_LINE_COL: (u32, u32) = (0, 0);

/// Get multiple pairs of lines and columns may be faster
///
/// Like [`line_column`]
///
/// # Panics
///
/// - index out of `0..s.len()`
/// - index not on char boundary
#[must_use]
#[track_caller]
pub fn line_columns<const N: usize>(
    s: &str,
    indexs: [usize; N],
) -> [(u32, u32); N] {
    let len = s.len();

    for index in indexs {
        assert!(index <= len,
                "index {index} out of str length {len} of `{s:?}`");
        assert!(s.is_char_boundary(index),
                "byte index {index} is not a char boundary of `{s:?}`");
    }

    let result = line_columns_unchecked(s, indexs);

    debug_assert!(! result.contains(&UNINIT_LINE_COL),
                  "impl error, report bug issue");
    result
}

/// Get multiple pairs of lines and columns may be faster
///
/// Like [`char_line_column`]
///
/// # Panics
/// - `indexs` any index greater than `s.chars().count()`
#[must_use]
#[track_caller]
pub fn char_line_columns<const N: usize>(
    s: &str,
    indexs: [usize; N],
) -> [(u32, u32); N] {
    let mut len = 0;
    let mut result = [UNINIT_LINE_COL; N];

    let last_loc = s.chars()
        .enumerate()
        .inspect(|&(i, _)| len = i+1)
        .fold((1, 1), |(line, column), (cur, ch)|
    {
        for (i, &index) in indexs.iter().enumerate() {
            if index == cur {
                result[i] = (line, column);
            }
        }

        if ch == '\n' {
            (line+1, 1)
        } else {
            (line, column+1)
        }
    });

    for index in indexs {
        assert!(index <= len,
                "char index {index} out of str length {len} of `{s:?}`");
    }

    for (i, &index) in indexs.iter().enumerate() {
        if index >= len {
            result[i] = last_loc;
        }
    }

    result
}

/// Get multiple of lines and columns may be faster
///
/// Use byte index
///
/// If the index does not fall on the character boundary,
/// the unspecified results
#[must_use]
pub fn line_columns_unchecked<const N: usize>(
    s: &str,
    indexs: [usize; N],
) -> [(u32, u32); N] {
    let len = s.len();
    let mut result = [UNINIT_LINE_COL; N];

    let last_loc = s.char_indices()
        .fold((1, 1), |(line, column), (cur, ch)|
    {
        for (i, &index) in indexs.iter().enumerate() {
            if index == cur {
                result[i] = (line, column);
            }
        }

        if ch == '\n' {
            (line+1, 1)
        } else {
            (line, column+1)
        }
    });

    for (i, &index) in indexs.iter().enumerate() {
        if index == len {
            result[i] = last_loc;
        }
    }

    result
}

/// Get str byte index of line and column
///
/// If the line or column out the length of the `s`, return `s.len()`
///
/// # Panics
/// - line or column by zero
///
/// # Examples
/// ```
/// # use line_column::index;
/// assert_eq!(index("", 1, 1),             0);
/// assert_eq!(index("a", 1, 1),            0);
/// assert_eq!(index("a", 1, 2),            1);
/// assert_eq!(index("a\n", 1, 2),          1);
/// assert_eq!(index("a\n", 2, 1),          2);
/// assert_eq!(index("a\nx", 2, 2),         3);
/// assert_eq!(index("你好\n世界", 1, 2),   3); // byte index
/// assert_eq!(index("你好\n世界", 1, 3),   6);
/// assert_eq!(index("你好\n世界", 2, 1),   7);
/// ```
#[must_use]
#[track_caller]
pub fn index(s: &str, line: u32, column: u32) -> usize {
    assert_ne!(line, 0);
    assert_ne!(column, 0);

    let mut i = 0;
    for _ in 1..line {
        let Some(lf) = s[i..].find('\n') else { break };
        i += lf+1;
    }
    let s = &s[i..];
    let lf = s.find('\n').map_or(s.len(), |l| l+1);
    let s = &s[..lf];
    i + s.char_indices()
        .nth(column as usize-1)
        .map_or(s.len(), |x| x.0)
}

/// Get str char index of line and column
///
/// If the line or column out the length of the `s`, return `s.chars().count()`
///
/// # Panics
/// - line or column by zero
///
/// # Examples
/// ```
/// # use line_column::char_index;
/// assert_eq!(char_index("", 1, 1),            0);
/// assert_eq!(char_index("a", 1, 1),           0);
/// assert_eq!(char_index("你好\n世界", 1, 2),  1);
/// assert_eq!(char_index("你好\n世界", 1, 3),  2);
/// assert_eq!(char_index("你好\n世界", 2, 1),  3);
/// ```
#[must_use]
#[track_caller]
pub fn char_index(s: &str, mut line: u32, mut column: u32) -> usize {
    assert_ne!(line, 0);
    assert_ne!(column, 0);

    line -= 1;
    column -= 1;

    let mut i = 0;
    let mut eol = false;

    for ch in s.chars() {
        if line == 0 {
            if column == 0 || eol { break }
            column -= 1;
            eol = ch == '\n';
        } else if ch == '\n' {
            line -= 1;
        }
        i += 1;
    }
    i
}

/// Get tuple of line and column, use byte index
///
/// Use LF (0x0A) to split newline, also compatible with CRLF (0x0D 0x0A)
///
/// # Examples
/// ```
/// # use line_column::line_column;
/// assert_eq!(line_column("", 0),     (1, 1));
/// assert_eq!(line_column("a", 0),    (1, 1));
/// assert_eq!(line_column("a", 1),    (1, 2));
/// assert_eq!(line_column("ab", 1),   (1, 2));
/// assert_eq!(line_column("a\n", 1),  (1, 2));
/// assert_eq!(line_column("a\n", 2),  (2, 1));
/// assert_eq!(line_column("a\nb", 2), (2, 1));
/// ```
#[inline]
#[must_use]
#[track_caller]
pub fn line_column(s: &str, index: usize) -> (u32, u32) {
    line_columns(s, [index])[0]
}

/// Get tuple of line and column, use char index
///
/// Use LF (0x0A) to split newline, also compatible with CRLF (0x0D 0x0A)
///
/// # Panics
/// - `index > s.chars().count()`
///
/// # Examples
/// ```
/// # use line_column::char_line_column;
/// assert_eq!(char_line_column("", 0),         (1, 1));
/// assert_eq!(char_line_column("a", 0),        (1, 1));
/// assert_eq!(char_line_column("a", 1),        (1, 2));
/// assert_eq!(char_line_column("ab", 1),       (1, 2));
/// assert_eq!(char_line_column("😀\n", 1),     (1, 2));
/// assert_eq!(char_line_column("😀\n", 2),     (2, 1));
/// assert_eq!(char_line_column("😀\n❓", 2),   (2, 1));
/// assert_eq!(char_line_column("😀\n❓", 2),   (2, 1));
/// ```
#[inline]
#[must_use]
#[track_caller]
pub fn char_line_column(s: &str, index: usize) -> (u32, u32) {
    char_line_columns(s, [index])[0]
}
