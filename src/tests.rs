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
        ("你好", 0, 1, 1),
        ("你好", 3, 1, 2),
        ("你好", 6, 1, 3),
        ("你好\n", 6, 1, 3),
        ("你好\n", 7, 2, 1),
    ];

    for (s, index, line, column) in tests {
        let result = line_column(s, index);
        assert_eq!(result, (line, column), "{s:?}[{index}]");
    }
}

#[test]
fn test_simple_char_index() {
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
        ("你好", 0, 1, 1),
        ("你好", 1, 1, 2),
        ("你好", 2, 1, 3),
        ("你好\n", 2, 1, 3),
        ("你好\n", 3, 2, 1),
        ("你\n好", 2, 2, 1),
        ("你\n好", 3, 2, 2),
    ];

    for (s, index, line, column) in tests {
        let result = char_line_column(s, index);
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

#[test]
fn index_test_pair() {
    let tests = [
        ("", 1, 1, 0),
        ("", 4, 4, 0),
        ("a", 1, 1, 0),
        ("a", 1, 2, 1),
        ("a", 3, 4, 1),
        ("\n", 1, 1, 0),
        ("\n", 1, 2, 0),
        ("a\n", 1, 1, 0),
        ("a\n", 1, 2, 1),
        ("a\n", 1, 3, 1),
        ("a\n", 2, 1, 2),
        ("a\n", 2, 2, 2),
        ("a\n", 2, 3, 2),
        ("a\n", 2, 3, 2),
        ("a\na", 2, 1, 2),
        ("a\na", 2, 2, 3),
        ("a\na", 2, 3, 3),
        ("a\n\n", 2, 1, 2),
        ("a\n\n", 2, 2, 2),
        ("a\n\n", 2, 3, 2),
        ("a\n\n", 3, 1, 3),
        ("a\n\n", 3, 2, 3),
        ("a\n\n", 3, 3, 3),
        ("a\n\nx", 2, 1, 2),
        ("a\n\nx", 2, 2, 2),
        ("a\n\nx", 2, 3, 2),
        ("a\nab\n", 2, 1, 2),
        ("a\nab\n", 2, 2, 3),
        ("a\nab\n", 2, 3, 4),
        ("a\nab\n", 2, 4, 4),
        ("a\nab\n", 2, 5, 4),
        ("a\nab\n", 3, 1, 5),
        ("a\nab\n", 3, 2, 5),
        ("a\nab\n", 3, 3, 5),
        ("a\nab\nx", 3, 1, 5),
        ("a\nab\nx", 3, 2, 6),
        ("a\nab\nx", 3, 3, 6),
    ];

    for d @ (src, line, column, expected) in tests {
        assert_eq!(index(src, line, column), expected, "(byte){d:?}");
        assert_eq!(char_index(src, line, column), expected, "(char){d:?}");
    }
}

#[test]
fn index_test() {
    let tests = [
        ("你好", 1, 1, 0),
        ("你好", 1, 2, 3),
        ("你好", 1, 3, 6),
        ("\n你好", 2, 1, 1),
        ("\n你好", 2, 2, 4),
        ("\n你好", 2, 3, 7),
    ];

    for d @ (src, line, column, expected) in tests {
        assert_eq!(index(src, line, column), expected, "{d:?}");
    }
}

#[test]
fn char_index_test() {
    let tests = [
        ("你好", 1, 1, 0),
        ("你好", 1, 2, 1),
        ("你好", 1, 3, 2),
        ("\n你好", 2, 1, 1),
        ("\n你好", 2, 2, 2),
        ("\n你好", 2, 3, 3),
    ];

    for d @ (src, line, column, expected) in tests {
        assert_eq!(char_index(src, line, column), expected, "{d:?}");
    }
}

#[test]
fn index_max_bound_test_pair() {
    let tests = [
        ("", 1, 1, 0),
        ("", 1, 2, 0),
        ("", 1, 3, 0),
        ("a", 1, 1, 0),
        ("a", 1, 2, 1),
        ("a", 1, 3, 1),
        ("\n", 1, 1, 0),
        ("\n", 1, 2, 0),
        ("\n", 1, 3, 0),
        ("\n$", 1, 1, 0),
        ("\n$", 1, 2, 0),
        ("\n$", 1, 3, 0),
        ("\n\n", 1, 1, 0),
        ("\n\n", 1, 2, 0),
        ("\n\n", 1, 3, 0),
        ("\n.\n", 1, 1, 0),
        ("\n.\n", 1, 2, 0),
        ("\n.\n", 1, 3, 0),
        ("m\n.\n", 1, 1, 0),
        ("m\n.\n", 1, 2, 1),
        ("m\n.\n", 1, 3, 1),
        ("\n.", 2, 1, 1),
        ("\n.", 2, 2, 2),
        ("\n.", 2, 3, 2),
        ("\n.", 2, 4, 2),
        ("\n.\n", 2, 1, 1),
        ("\n.\n", 2, 2, 2),
        ("\n.\n", 2, 3, 2),
        ("\n.\n", 2, 4, 2),
        ("\n.\n$", 2, 1, 1),
        ("\n.\n$", 2, 2, 2),
        ("\n.\n$", 2, 3, 2),
        ("\n.\n$", 2, 4, 2),
        ("\n.\n", 3, 1, 3),
        ("\n.\n", 3, 2, 3),
        ("\n.\n", 3, 3, 3),
        ("\n.\n$", 3, 1, 3),
        ("\n.\n$", 3, 2, 4),
        ("\n.\n$", 3, 3, 4),
        ("\n.\n$", 3, 4, 4),
    ];

    for d @ (src, line, column, expected)
        in tests
    {
        assert_eq!(index(src, line, column), expected, "(byte){d:?}");
        assert_eq!(char_index(src, line, column), expected, "(char){d:?}");
    }
}
