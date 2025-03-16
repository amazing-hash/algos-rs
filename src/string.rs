use std::collections::{BTreeMap, VecDeque};

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

#[allow(unused)]
pub fn z_function(src: &str) -> Vec<usize> {
    let mut z = vec![0; src.len()];
    let bytes = src.as_bytes();
    let mut l = 0usize;
    let mut r = 0usize;
    for i in 1..src.len() {
        if i <= r {
            z[i] = std::cmp::min(r - i + 1, z[i - l]);
        }
        while i + z[i] < src.len() && bytes[z[i]] == bytes[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }
    z
}

#[cfg(test)]
#[test]
fn test_z_function_ascii() {
    assert_eq!(z_function("abacaba"), [0, 0, 1, 0, 3, 0, 1]);
}

#[allow(unused)]
pub fn hash(s: &str) -> u64 {
    let mut k = 1;
    let mut res = 0u64;
    for ch in s.as_bytes() {
        res = res.wrapping_add((*ch as u64).wrapping_mul(k));
        k = k.wrapping_mul(31);
    }
    res
}

#[allow(unused)]
pub fn hash2(s: &str) -> u64 {
    let mut k = 1;
    let mut res = 0u64;
    for ch in s.as_bytes() {
        res = res.wrapping_mul(31).wrapping_add(*ch as u64);
    }
    res
}

#[cfg(test)]
#[test]
fn test_hash() {
    assert_eq!(hash(""), 0);
    assert_eq!(hash("a"), 97);
    assert_eq!(hash("abc"), 98274);
    assert_eq!(hash("abcdabcd"), 2842022591228);
    assert_eq!(hash("abcd"), hash("abcd"));
}

#[allow(unused)]
pub fn rabin_karp(t: &str, p: &str) -> Vec<usize> {
    if t.is_empty() || p.is_empty() {
        return vec![];
    }
    let bytes = t.as_bytes();
    let mut res = vec![];
    let hash_p = hash2(p);
    let mut hash_t = hash2(&t[0..p.len()]);
    for i in 0..t.len() - p.len() {
        if hash_p == hash_t {
            res.push(i);
        }
        hash_t = hash_t
            .wrapping_mul(31u64)
            .wrapping_add(bytes[i + p.len()] as u64)
            .wrapping_sub((bytes[i] as u64).wrapping_mul(31u64.wrapping_pow(p.len() as u32)));
    }
    if hash_p == hash_t {
        res.push(t.len() - p.len());
    }
    res
}

#[cfg(test)]
#[test]
fn test_rabin_karp() {
    assert_eq!(rabin_karp("abcxabcxyabcxyz", "abc"), vec![0, 4, 9]);
    assert_eq!(rabin_karp("ababaac", "bab"), vec![1]);
    assert_eq!(rabin_karp("ababcxabdabcxabcxabcde", "abcxabcde"), vec![13]);
}

#[allow(unused)]
pub fn kmp(t: &str, p: &str) -> Vec<usize> {
    if t.is_empty() || p.is_empty() {
        return vec![];
    }
    let mut res = vec![];
    let prefix = prefix_function(p);
    let bytes = p.as_bytes();
    let mut idx = 0;
    for (i, value) in t.as_bytes().iter().enumerate() {
        while idx > 0 && bytes[idx] != *value {
            idx = prefix[idx - 1];
        }
        if bytes[idx] == *value {
            idx += 1;
        }
        if idx == p.len() {
            res.push(i + 1 - idx);
            idx = prefix[idx - 1];
        }
    }
    res
}

#[cfg(test)]
#[test]
fn test_kmp() {
    assert_eq!(kmp("ababcxabdabcxabcxabcde", "abcxabcde"), vec![13]);
    assert_eq!(kmp("a", "ab"), vec![]);
    assert_eq!(kmp("", ""), vec![]);
    assert_eq!(kmp("aaaaa", "a"), vec![0, 1, 2, 3, 4]);
    assert_eq!(kmp("abcdabcd", "abc"), vec![0, 4]);
}

#[allow(unused)]
pub fn compress(s: &str) -> (&str, usize) {
    let k = s.len() - prefix_function(s)[s.len() - 1];
    match s.len() % k {
        0 => (&s[..k], s.len() / k),
        _ => (s, 1),
    }
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(compress("abcabcabc"), ("abc", 3));
    assert_eq!(compress("abcd"), ("abcd", 1));
}

#[allow(unused)]
pub fn levenshtein_distance(
    first: &str,
    second: &str,
    delete_cost: u32,
    insert_cost: u32,
    replace_cost: u32,
) -> u32 {
    let first = first.as_bytes();
    let second = second.as_bytes();
    let mut dist = vec![0; first.len() + 1];
    for j in 1..(first.len() + 1) {
        dist[j] = dist[j - 1] + insert_cost;
    }
    for i in 1..second.len() + 1 {
        let mut dist_temp = vec![0; first.len() + 1];
        dist_temp[0] = dist[0] + delete_cost;
        for j in 1..(first.len() + 1) {
            if second[i - 1] != first[j - 1] {
                dist_temp[j] = std::cmp::min(
                    std::cmp::min(dist[j] + delete_cost, dist[j - 1] + insert_cost),
                    dist_temp[j - 1] + replace_cost,
                );
            } else {
                dist_temp[j] = dist[j - 1];
            }
        }
        dist = dist_temp;
    }
    dist[first.len()]
}

#[cfg(test)]
#[test]
fn test_levenshtein_distance() {
    assert_eq!(
        levenshtein_distance("POLYNOMIAL", "EXPONENTIAL", 1, 1, 1),
        6
    );
    assert_eq!(levenshtein_distance("abcdasdasd", "cddabcd", 1, 1, 1), 6);
    assert_eq!(levenshtein_distance("", "", 1, 1, 1), 0);
    assert_eq!(levenshtein_distance("aaa", "aaa", 1, 1, 1), 0);
    assert_eq!(levenshtein_distance("", "aaa", 1, 1, 1), 3);
}

#[allow(unused)]
#[derive(Clone)]
struct VertexAhoCorasick {
    children: BTreeMap<i32, i32>,
    link: i32,
    good_link: i32,
    pat_num: i32,
    pch: i32,
    parent: i32,
}

#[allow(unused)]
struct TrieAhoCorasick {
    arr: Vec<VertexAhoCorasick>,
    sz: i32,
}

#[allow(unused)]
impl TrieAhoCorasick {
    fn new() -> TrieAhoCorasick {
        TrieAhoCorasick {
            arr: vec![
                VertexAhoCorasick {
                    children: BTreeMap::new(),
                    link: -1,
                    good_link: -1,
                    pat_num: -1,
                    pch: -1,
                    parent: -1
                };
                1
            ],
            sz: 1,
        }
    }
    fn insert(&mut self, s: &str, num: i32) {
        let mut v = 0;
        for ch in s.as_bytes() {
            let idx = *ch as i32;
            #[allow(clippy::map_entry)]
            if !self.arr[v].children.contains_key(&idx) {
                self.arr.push(VertexAhoCorasick {
                    children: BTreeMap::new(),
                    link: 0,
                    good_link: -1,
                    pat_num: -1,
                    pch: idx,
                    parent: v as i32,
                });
                self.arr[v].children.insert(idx, self.sz);
                self.sz += 1;
            }
            v = *self.arr[v].children.get(&idx).unwrap() as usize;
        }
        self.arr[v].pat_num = num;
    }
}

#[allow(unused)]
pub fn aho_corasick(dict: &[&str], t: &str) -> BTreeMap<i32, Vec<usize>> {
    let mut res: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
    let mut trie = TrieAhoCorasick::new();
    for (idx, s) in dict.iter().enumerate() {
        trie.insert(s, idx as i32);
    }
    let mut q = VecDeque::new();
    q.push_back(0);
    while !q.is_empty() {
        let curr = q.pop_front().unwrap();

        for value in trie.arr[curr as usize].children.values() {
            q.push_back(*value);
        }
        if curr == 0 {
            continue;
        }
        let parent = trie.arr[curr as usize].parent;
        let mut next_link = trie.arr[parent as usize].link;
        let pch = trie.arr[curr as usize].pch;
        while next_link >= 0 && !trie.arr[next_link as usize].children.contains_key(&pch) {
            next_link = trie.arr[next_link as usize].link;
        }
        if next_link >= 0 {
            let link = *trie.arr[next_link as usize].children.get(&pch).unwrap();
            let good_link = if trie.arr[link as usize].pat_num != -1 {
                link
            } else {
                trie.arr[link as usize].good_link
            };
            let r = &mut trie.arr[curr as usize];
            r.link = link;
            r.good_link = good_link;
        }
    }
    let mut v = 0i32;
    for (i, ch) in t.as_bytes().iter().enumerate() {
        let idx = *ch as i32;
        while v >= 0 && !trie.arr[v as usize].children.contains_key(&idx) {
            v = trie.arr[v as usize].link;
        }
        if v == -1 {
            v = 0;
        } else {
            v = *trie.arr[v as usize].children.get(&idx).unwrap();
        }
        if trie.arr[v as usize].pat_num != -1 {
            let value = res.entry(trie.arr[v as usize].pat_num).or_default();
            (*value).push(i + 1 - dict[trie.arr[v as usize].pat_num as usize].len());
        }
        let mut good_link = trie.arr[v as usize].good_link;
        while good_link > 0 {
            let value = res.entry(trie.arr[good_link as usize].pat_num).or_default();
            (*value).push(i + 1 - dict[trie.arr[good_link as usize].pat_num as usize].len());
            good_link = trie.arr[good_link as usize].good_link;
        }
    }
    res
}

#[test]
fn aho_corasick_test() {
    let mut dict = ["aba", "abb", "bbca"];
    let text = "abaabbbbca";
    let mut res = aho_corasick(&dict, text);

    let mut map = BTreeMap::new();
    map.insert(0, vec![0]);
    map.insert(1, vec![3]);
    map.insert(2, vec![6]);
    assert_eq!(map, res);

    let text = "abaabbbbcaaba";
    res = aho_corasick(&dict, text);

    map = BTreeMap::new();
    map.insert(0, vec![0, 10]);
    map.insert(1, vec![3]);
    map.insert(2, vec![6]);
    assert_eq!(map, res);

    let text = "abaabbbbcaba";
    res = aho_corasick(&dict, text);

    map = BTreeMap::new();
    map.insert(0, vec![0, 9]);
    map.insert(1, vec![3]);
    map.insert(2, vec![6]);
    assert_eq!(map, res);

    dict = ["abba", "bb", "cc"];
    let text = "abba";
    res = aho_corasick(&dict, text);

    map = BTreeMap::new();
    map.insert(0, vec![0]);
    map.insert(1, vec![1]);
    assert_eq!(map, res);

    dict = ["aba", "baba", "cc"];
    let text = "ababababa";
    res = aho_corasick(&dict, text);

    map = BTreeMap::new();
    map.insert(0, vec![0, 2, 4, 6]);
    map.insert(1, vec![1, 3, 5]);
    assert_eq!(map, res);

    let dict = ["aba", "baba", "cc"];
    let text = "ababababa";
    let res = aho_corasick(&dict, text);

    let mut map = BTreeMap::new();
    map.insert(0, vec![0, 2, 4, 6]);
    map.insert(1, vec![1, 3, 5]);
    assert_eq!(map, res);
}

#[allow(unused)]
pub fn diferent_substrings_generator(s: &str) -> Vec<&str> {
    let mut seq = vec![];
    for i in (0..s.len()).rev() {
        let pr = z_function(&s[i..s.len()]);
        let res = s.len() - i - pr.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        for j in 0..res {
            seq.push(&s[i..s.len() - j]);
        }
    }
    seq
}

#[allow(unused)]
pub fn distinct_substrings_count(s: &str) -> usize {
    let mut cnt = 0;
    for i in (0..s.len()).rev() {
        let pr = z_function(&s[i..s.len()]);
        let res = s.len() - i - pr.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        cnt += res;
    }
    cnt
}

#[cfg(test)]
#[test]
fn test_distinct_substrings() {
    assert_eq!(diferent_substrings_generator("a"), vec!["a"]);
    assert_eq!(
        diferent_substrings_generator("aaaa"),
        vec!["a", "aa", "aaa", "aaaa"]
    );
    assert_eq!(diferent_substrings_generator(""), Vec::<&str>::new());
    let mut values = diferent_substrings_generator("abaaba");
    values.sort();
    assert_eq!(
        values,
        vec![
            "a", "aa", "aab", "aaba", "ab", "aba", "abaa", "abaab", "abaaba", "b", "ba", "baa",
            "baab", "baaba"
        ]
    );
    assert_eq!(distinct_substrings_count("abacabadabacaba"), 85);
}

#[allow(unused)]
pub fn find_minimum_string_period(src: &str) -> &str {
    for (idx, value) in z_function(src).iter().enumerate() {
        if value + idx == src.len() && src.len() % idx == 0 {
            return &src[..idx];
        } else if value + idx == src.len() {
            let k = src.len() % idx;
            if src[..k] == src[src.len() - k..] {
                return &src[..idx];
            }
        }
    }
    src
}

#[test]
fn test_minimum_string_period() {
    assert_eq!(find_minimum_string_period("abcabcabca"), "abc");
    assert_eq!(find_minimum_string_period("abcdefg"), "abcdefg");
    assert_eq!(find_minimum_string_period("abcabcabcd"), "abcabcabcd");
    assert_eq!(find_minimum_string_period(""), "");
}
