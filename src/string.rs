#[allow(unused)]
pub fn prefix_function(src: &str) -> Vec<usize> {
    if src.is_empty() {
        return vec![];
    }
    let mut prefix = vec![0; src.len()];
    let bytes = src.as_bytes();
    for i in 1..src.len() {
        let mut j = prefix[i - 1];
        while j > 0 && bytes[i] != bytes[j] {
            j = prefix[j - 1];
        }
        if bytes[i] == bytes[j] {
            j += 1;
        }
        prefix[i] = j;
    }
    prefix
}

#[cfg(test)]
#[test]
fn test_prefix_function() {
    assert_eq!(prefix_function("abacaba"), [0, 0, 1, 0, 1, 2, 3]);
    assert_eq!(prefix_function("b"), [0]);
    assert_eq!(prefix_function("aaaaa"), [0, 1, 2, 3, 4]);
    assert_eq!(prefix_function(""), []);
}
