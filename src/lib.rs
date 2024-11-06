#![no_std]
#![doc = include_str!("../README.md")]
#[cfg(test)]
mod tests;

const UNINIT_LINE_COL: (u32, u32) = (0, 0);

/// Get multiple sets of lines and columns may be faster
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

/// Get multiple of lines and columns may be faster
///
/// If the index does not fall on the character boundary,
/// the unspecified results
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

/// Get tuple of line and column
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
pub fn line_column(s: &str, index: usize) -> (u32, u32) {
    line_columns(s, [index])[0]
}
