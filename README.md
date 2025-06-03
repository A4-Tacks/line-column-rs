Simple calculate lines and columns of str index

Use LF (0x0A) to split newline, also compatible with CRLF (0x0D 0x0A)

Newline char line number is current line

# Examples

**Byte index to line number**:

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

**Character index to line number**:

```rust
use line_column::char_line_column;

assert_eq!(char_line_column("", 0),         (1, 1));
assert_eq!(char_line_column("a", 0),        (1, 1));
assert_eq!(char_line_column("a", 1),        (1, 2));
assert_eq!(char_line_column("ab", 1),       (1, 2));
assert_eq!(char_line_column("😀\n", 1),     (1, 2));
assert_eq!(char_line_column("😀\n", 2),     (2, 1));
assert_eq!(char_line_column("😀\n❓", 2),   (2, 1));
assert_eq!(char_line_column("😀\n❓", 2),   (2, 1));
```

**Line number to byte index**:

```rust
use line_column::index;

assert_eq!(index("", 1, 1),             0);
assert_eq!(index("a", 1, 1),            0);
assert_eq!(index("a", 1, 2),            1);
assert_eq!(index("a\n", 1, 2),          1);
assert_eq!(index("a\n", 2, 1),          2);
assert_eq!(index("a\nx", 2, 2),         3);
assert_eq!(index("你好\n世界", 1, 2),   3); // byte index
assert_eq!(index("你好\n世界", 1, 3),   6);
assert_eq!(index("你好\n世界", 2, 1),   7);
```

**Line number to character index**:

```rust
use line_column::char_index;

assert_eq!(char_index("", 1, 1),            0);
assert_eq!(char_index("a", 1, 1),           0);
assert_eq!(char_index("你好\n世界", 1, 2),  1);
assert_eq!(char_index("你好\n世界", 1, 3),  2);
assert_eq!(char_index("你好\n世界", 2, 1),  3);
```

**The end of string is considered a character**:

```rust
use line_column::*;

assert_eq!(index("", 1, 1),             0);
assert_eq!(index("a", 1, 2),            1);
assert_eq!(char_index("", 1, 1),        0);
assert_eq!(char_index("a", 1, 2),       1);
assert_eq!(line_column("", 0),          (1, 1));
assert_eq!(line_column("a", 1),         (1, 2));
assert_eq!(char_line_column("", 0),     (1, 1));
assert_eq!(char_line_column("a", 1),    (1, 2));
```
