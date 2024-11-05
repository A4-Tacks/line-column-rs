use crate::*;

#[test]
fn test_simple() {
    let tests = [
        ("", 0, 1, 1),
        ("a", 0, 1, 1),
        ("\n", 0, 1, 1),
        ("a", 1, 1, 2),
        ("aa", 1, 1, 2),
        ("a\n", 1, 1, 2),
        ("\n", 1, 2, 1),
        ("\na", 1, 2, 1),
        ("\n\n", 1, 2, 1),
        ("\n\n", 2, 3, 1),
    ];

    for (s, index, line, column) in tests {
        let result = line_column(s, index);
        assert_eq!(result, (line, column), "{s:?}[{index}]");
    }
}

#[test]
fn test_crlf_simple() {
    let tests = [
        ("", 0, 1, 1),
        ("a", 0, 1, 1),
        ("\r\n", 0, 1, 1),
        ("\r\n", 1, 1, 2),
        ("\r\n", 2, 2, 1),
        ("a", 1, 1, 2),
        ("aa", 1, 1, 2),
        ("a\r\n", 1, 1, 2),
        ("a\r\n", 2, 1, 3),
        ("a\r\n", 3, 2, 1),
        ("\r\n", 1, 1, 2),
        ("\r\n", 2, 2, 1),
        ("\r\na", 1, 1, 2),
        ("\r\na", 2, 2, 1),
        ("\r\n\r\n", 1, 1, 2),
        ("\r\n\r\n", 2, 2, 1),
        ("\r\n\r\n", 3, 2, 2),
        ("\r\n\r\n", 4, 3, 1),
    ];

    for (s, index, line, column) in tests {
        let result = line_column(s, index);
        assert_eq!(result, (line, column), "{s:?}[{index}]");
    }
}

#[test]
fn test_mult() {
    let tests = [
        ("a",       [0, 1],             1, 1,               1, 2),
        ("\n",      [0, 0],             1, 1,               1, 1),
        ("a",       [1, 1],             1, 2,               1, 2),
        ("aa",      [1, 2],             1, 2,               1, 3),
        ("a\n",     [1, 2],             1, 2,               2, 1),
        ("\n",      [0, 1],             1, 1,               2, 1),
        ("\na",     [1, 1],             2, 1,               2, 1),
        ("\n\n",    [1, 2],             2, 1,               3, 1),
        ("\n\n",    [2, 2],             3, 1,               3, 1),
    ];

    for (s, indexs, l1, c1, l2, c2) in tests {
        let result = line_columns(s, indexs);
        assert_eq!(result, [(l1, c1), (l2, c2)], "{s:?}{indexs:?}");
    }
}
