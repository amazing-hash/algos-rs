#[allow(unused)]
pub fn binary_search<T>(container: &[T], key: &T) -> Option<usize>
where
    T: std::cmp::Ord,
{
    if container.is_empty() {
        return None;
    }
    let mut l = 0isize;
    let mut r = isize::try_from(container.len() - 1).unwrap();
    while l < r {
        let idx = l + (r - l) / 2;
        if container[idx as usize] == *key {
            return Some(idx as usize);
        }
        if container[idx as usize] < *key {
            l = idx + 1;
        } else {
            r = idx - 1;
        }
    }
    match container[l as usize] == *key {
        true => Some(l as usize),
        _ => None,
    }
}

#[allow(unused)]
pub fn binary_search_comp<T, F>(container: &[T], key: &T, comp: F) -> Option<usize>
where
    T: std::cmp::Ord, F: Fn(&T, &T) -> std::cmp::Ordering,
{
    if container.is_empty() {
        return None;
    }
    let mut l = 0isize;
    let mut r = isize::try_from(container.len() - 1).unwrap();
    while l < r {
        let idx = l + (r - l) / 2;
        match comp(key, &container[idx as usize]) {
            std::cmp::Ordering::Equal => {
                return Some(idx as usize);
            }
            std::cmp::Ordering::Greater => {
                l = idx + 1;
            }
            std::cmp::Ordering::Less => {
                r = idx - 1;
            }
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
    let mut r = isize::try_from(container.len() - 1).unwrap();
    while l < r {
        let idx = l + (r - l) / 2;
        if *key <= container[idx as usize] {
            r = idx;
        } else {
            l = idx + 1;
        }
    }
    if (l as usize) < container.len() && container[l as usize] < *key {
        l += 1;
    }
    l as usize
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
    let mut r = isize::try_from(container.len() - 1).unwrap();
    while l < r {
        let idx = l + (r - l) / 2;
        if *key >= container[idx as usize] {
            l = idx + 1;
        } else {
            r = idx;
        }
    }
    if (l as usize) < container.len() && container[l as usize] <= *key {
        l += 1;
    }
    l as usize
}

#[cfg(test)]
#[test]
fn binary_search_test() {
    let seq = vec![1, 2, 3, 4, 5, 8, 9, 20];
    assert_eq!(binary_search(&seq, &8).unwrap(), 5);
    assert_eq!(binary_search(&seq, &1).unwrap(), 0);
    assert_eq!(binary_search(&seq, &7), None);
    assert_eq!(binary_search(&seq, &21), None);

    let seq = vec![1];
    assert_eq!(binary_search(&seq, &1).unwrap(), 0);

    let seq = vec![1, 2, 3, 4, 5];
    assert_eq!(binary_search(&seq, &5).unwrap(), 4);

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

    let seq = vec![1, 2, 3, 4, 5, 8, 9, 20];
    let comp = |key: &i32, other: &i32| {
        key.cmp(other)
    };
    assert_eq!(binary_search_comp(&seq, &8, comp).unwrap(), 5);
    assert_eq!(binary_search_comp(&seq, &1, comp).unwrap(), 0);
    assert_eq!(binary_search_comp(&seq, &7, comp), None);
    assert_eq!(binary_search_comp(&seq, &21, comp), None);
}
