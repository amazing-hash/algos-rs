#[allow(unused)]
pub fn binary_search<T>(container: &[T], key: &T) -> Option<usize>
where
    T: std::cmp::Ord,
{
    if container.is_empty() {
        return None;
    }
    let mut l = 0isize;
    let mut r = container.len() as isize - 1;
    while l < r {
        let idx = (l + r) / 2;
        if container[idx as usize] == *key {
            return Some(idx as usize);
        }
        if container[idx as usize] < *key {
            l = idx + 1;
        } else {
            r = idx - 1;
        }
        if l > r {
            return None;
        }
    }

    match container[l as usize] == *key {
        true => Some(l as usize),
        _ => None,
    }
}

#[allow(unused)]
pub fn lower_bound<T>(container: &[T], key: &T) -> usize
where
    T: std::cmp::Ord,
{
    if container.is_empty() {
        return 0;
    }
    let mut l = 0;
    let mut r = container.len() - 1;
    while l < r {
        let idx = l + (r - l) / 2;
        if *key <= container[idx] {
            r = idx;
        } else {
            l = idx + 1;
        }
    }
    if l < container.len() && container[l] < *key {
        l += 1;
    }
    l
}

#[allow(unused)]
pub fn lower_bound_comp<F>(mut l: usize, mut r: usize, comp: F) -> usize
where
    F: Fn(usize) -> bool,
{
    while l < r {
        let idx = l + (r - l) / 2;
        if comp(idx) {
            r = idx;
        } else {
            l = idx + 1;
        }
    }
    l
}

#[allow(unused)]
pub fn upper_bound<T>(container: &[T], key: &T) -> usize
where
    T: std::cmp::Ord,
{
    if container.is_empty() {
        return 0;
    }
    let mut l = 0;
    let mut r = container.len() - 1;
    while l < r {
        let idx = l + (r - l) / 2;
        if *key >= container[idx] {
            l = idx + 1;
        } else {
            r = idx;
        }
    }
    if l < container.len() && container[l] <= *key {
        l += 1;
    }
    l
}

#[allow(unused)]
pub fn upper_bound_comp<F>(mut l: usize, mut r: usize, comp: F) -> usize
where
    F: Fn(usize) -> bool,
{
    while l < r {
        let idx = l + (r - l) / 2;
        if comp(idx) {
            l = idx + 1;
        } else {
            r = idx;
        }
    }
    l
}

#[cfg(test)]
#[test]
fn binary_search_test() {
    let seq = vec![1, 2, 3, 4, 5, 8, 9, 20];
    assert_eq!(binary_search(&seq, &8).unwrap(), 5);
    assert_eq!(binary_search(&seq, &1).unwrap(), 0);
    assert_eq!(binary_search(&seq, &7), None);
    assert_eq!(binary_search(&seq, &21), None);

    let seq = vec![1, 2, 3, 4, 5, 8, 8, 8, 9, 20];
    assert_eq!(lower_bound(&seq, &8), 5);
    assert_eq!(lower_bound(&seq, &1), 0);
    assert_eq!(lower_bound(&seq, &7), 5);
    assert_eq!(lower_bound(&seq, &21), seq.len());
    assert_eq!(lower_bound(&seq, &20), 9);

    let seq = vec![1];
    assert_eq!(lower_bound(&seq, &1), 0);

    let seq = vec![1, 2, 3, 4, 5, 8, 8, 8, 9, 20];
    assert_eq!(upper_bound(&seq, &8), 8);
    assert_eq!(upper_bound(&seq, &1), 1);
    assert_eq!(upper_bound(&seq, &7), 5);
    assert_eq!(upper_bound(&seq, &21), seq.len());

    let seq = vec![1];
    assert_eq!(upper_bound(&seq, &1), 1);

    let seq = vec![];
    assert_eq!(upper_bound(&seq, &1), 0);
    assert_eq!(lower_bound(&seq, &1), 0);
}
