Simple calculate lines and columns of str index

Use LF (0x0A) to split newline, also compatible with CRLF (0x0D 0x0A)

Newline char line number is current line

# Examples
```rust
use line_column::line_column;

assert_eq!(line_column("", 0),       (1, 1));
assert_eq!(line_column("a", 0),      (1, 1));
assert_eq!(line_column("a", 1),      (1, 2));
assert_eq!(line_column("ab", 1),     (1, 2));
assert_eq!(line_column("a\n", 1),    (1, 2));
assert_eq!(line_column("a\n", 2),    (2, 1));
assert_eq!(line_column("a\nb", 2),   (2, 1));
assert_eq!(line_column("a\r\nb", 2), (1, 3));
assert_eq!(line_column("a\r\nb", 3), (2, 1));
```
