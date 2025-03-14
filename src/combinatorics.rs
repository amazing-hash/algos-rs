#[allow(unused)]
pub fn next_permutation<T: Ord + Copy>(arr: &mut [T]) -> Option<()> {
    let ptr = arr.as_mut_ptr();
    for i in (1..arr.len()).rev() {
        if arr[i - 1] < arr[i] {
            for j in (i..arr.len()).rev() {
                if arr[j] > arr[i - 1] {
                    unsafe {
                        std::ptr::swap(ptr.offset(i as isize - 1), ptr.add(j));
                    }
                    arr[i..].reverse();
                    return Some(());
                }
            }
        }
    }
    None
}

#[cfg(test)]
#[test]
fn test_permutation() {
    let arr = vec![[0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
    let mut values = vec![0, 1, 2];
    let mut idx = 0;
    while let Some(_) = next_permutation(&mut values) {
        assert_eq!(vec![values[0], values[1], values[2]], arr[idx]);
        idx += 1;
    }
    assert_eq!(idx, arr.len());
}

#[allow(unused)]
pub fn next_subset(arr: &mut [bool], current: &mut usize) -> Option<()> {
    if arr.is_empty() {
        return None;
    }
    if *current < (1 << arr.len()) {
        for (j, item) in arr.iter_mut().enumerate() {
            let value = 1 << j;
            if *current & value == value {
                *item = true;
            } else {
                *item = false;
            }
        }
        *current += 1;
        return Some(());
    }
    None
}

#[test]
fn next_subset_test() {
    let values = vec![
        [false, false, false],
        [true, false, false],
        [false, true, false],
        [true, true, false],
        [false, false, true],
        [true, false, true],
        [false, true, true],
        [true, true, true],
    ];

    let mut set = vec![false; 3];
    let mut current = 0;
    let mut idx = 0;
    while let Some(_) = next_subset(&mut set, &mut current) {
        assert_eq!(values[idx], set[..]);
        idx += 1;
    }
}
