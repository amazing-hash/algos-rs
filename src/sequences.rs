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
