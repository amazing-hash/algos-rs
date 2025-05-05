/*
* Search for the nearest next smaller element for each element of the array
* asymptotics of O(n)
* nse is the next smallest element
 */

#[allow(unused)]
pub fn search_nearest_nse_for_each_element(arr: &[u32]) -> Vec<Option<usize>> {
    let mut st = Vec::new();
    let mut next = vec![None; arr.len()];
    for idx in 0..arr.len() {
        while !st.is_empty() && arr[idx] < arr[*st.last().unwrap()] {
            next[st.pop().unwrap()] = Some(idx);
        }
        st.push(idx);
    }
    next
}

#[cfg(test)]
#[test]
fn search_nearest_nse_for_each_element_test() {
    let array = &[4, 8, 5, 2, 25];
    assert_eq!(
        search_nearest_nse_for_each_element(array),
        vec![Some(3), Some(2), Some(3), None, None]
    );
    let array = &[];
    assert_eq!(search_nearest_nse_for_each_element(array), vec![]);
    let array = &[1];
    assert_eq!(search_nearest_nse_for_each_element(array), vec![None]);
    let array = &[1, 2, 3, 4, 5];
    assert_eq!(
        search_nearest_nse_for_each_element(array),
        vec![None, None, None, None, None]
    );
    let array = &[5, 4, 3, 2, 1];
    assert_eq!(
        search_nearest_nse_for_each_element(array),
        vec![Some(1), Some(2), Some(3), Some(4), None]
    );
}

/*
* Search for the nearest previous smaller element for each element of the array
* asymptotic of O(n)
* pse is the previous smaller element
*/
#[allow(unused)]
fn search_nearest_pse_for_each_element(arr: &[u32]) -> Vec<Option<usize>> {
    let mut st: Vec<Option<usize>> = Vec::new();
    let mut prev = vec![None; arr.len()];
    for idx in 0..arr.len() {
        while !st.is_empty() && arr[idx] <= arr[(st.last().unwrap()).unwrap()] {
            st.pop();
        }
        if !st.is_empty() {
            prev[idx] = *st.last().unwrap();
        }
        st.push(Some(idx));
    }
    prev
}
#[cfg(test)]
#[test]
fn search_nearest_pse_for_each_element_test() {
    let array = &[1, 6, 4, 10, 2, 5];
    assert_eq!(
        search_nearest_pse_for_each_element(array),
        vec![None, Some(0), Some(0), Some(2), Some(0), Some(4)]
    );
    let array = &[];
    assert_eq!(search_nearest_pse_for_each_element(array), vec![]);
    let array = &[1];
    assert_eq!(search_nearest_pse_for_each_element(array), vec![None]);
    let array = &[1, 2, 3, 4, 5];
    assert_eq!(
        search_nearest_pse_for_each_element(array),
        vec![None, Some(0), Some(1), Some(2), Some(3)]
    );
    let array = &[5, 4, 3, 2, 1];
    assert_eq!(
        search_nearest_pse_for_each_element(array),
        vec![None, None, None, None, None]
    );
}

// Search insert position of K in a sorted array
#[allow(unused)]
pub fn search_insert_pos(arr: &[i32], k: i32) -> usize {
    if arr.is_empty() {
        return 0;
    }
    let mut left = 0isize;
    let mut right = arr.len() as isize - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        // If k is found at mid
        if arr[mid as usize] == k {
            return mid as usize;
        }
        // If k is smaller, search in left half
        else if arr[mid as usize] > k {
            right = mid - 1;
        }
        // If k is larger, search in right half
        else {
            left = mid + 1;
        }
    }

    // If k is not found, return insert position
    return left as usize;
}

#[cfg(test)]
#[test]
fn search_insert_pos_test() {
    let array = &[1, 3, 5, 6];
    assert_eq!(search_insert_pos(array, 5), 2);
    let array = &[12];
    assert_eq!(search_insert_pos(array, 5), 0);
    let array = &[];
    assert_eq!(search_insert_pos(array, 5), 0);
}

// Search longest common subsequence
#[allow(unused)]
pub fn lcs(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut ans = vec![];
    let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }
    let mut i = a.len();
    let mut j = b.len();
    while i > 0 && j > 0  {
        if a[i - 1] == b[j - 1] {
            ans.push(a[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i][j] == dp[i][j - 1] {
            j -= 1;
        } else {
            i -= 1;
        }
    }
    ans.reverse();
    ans
}

#[cfg(test)]
#[test]
fn lcs_test() {
    let a = [1, 2, 3];
    let b = [3, 2, 1];
    assert_eq!(lcs(&a, &b), vec![3]);
    let a = [1, 100, 2, 100, 3];
    let b = [10, 10, 1, 2, 3, 10];
    assert_eq!(lcs(&a, &b), vec![1, 2, 3]);
}