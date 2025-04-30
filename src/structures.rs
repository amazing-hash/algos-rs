/*
* This data structure provides the following features. Initially, there are several elements, each of which is in a separate (its own) set.
* In one operation, you can combine any two sets, and you can also query which set the specified element is currently in.
* The asymptotics of all operations are on average O(1)
*/
pub struct Dsu {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Dsu {
            parents: (0..n).collect(),
            ranks: vec![1; n],
        }
    }

    /*
     * returns which set contains the specified element
     */
    pub fn lookup(&mut self, key: usize) -> Option<usize> {
        if key >= self.parents.len() {
            return None;
        }
        if key == self.parents[key] {
            return Some(key);
        }
        self.parents[key] = self.lookup(self.parents[key]).unwrap();
        Some(self.parents[key])
    }

    /*
     * combines the two specified sets (the set containing the first element and the set containing the second element)
     */
    pub fn union(&mut self, first: usize, second: usize) -> bool {
        if let (Some(first), Some(second)) = (self.lookup(first), self.lookup(second)) {
            if first != second {
                if self.ranks[first] < self.ranks[second] {
                    self.ranks.swap(first, second);
                }
                self.parents[second] = first;
                if self.ranks[first] == self.ranks[second] {
                    self.ranks[first] += 1;
                }
            }
            return true;
        }
        false
    }
}

#[cfg(test)]
#[test]
fn dsu_test() {
    let mut dsu = Dsu::new(10);

    dsu.union(1, 2);
    dsu.union(2, 3);
    dsu.union(2, 7);

    assert_eq!(dsu.lookup(2).unwrap(), dsu.lookup(7).unwrap());
    assert_eq!(dsu.lookup(1).unwrap(), dsu.lookup(3).unwrap());
    assert_ne!(dsu.lookup(1).unwrap(), dsu.lookup(8).unwrap());
    assert_eq!(dsu.lookup(9).unwrap(), 9);
}

/*
* A Bloom filter is a data structure whose purpose is to quickly verify,
* that the element is NOT included in the set (for those who are familiar with the O notation).,
* the difficulty of inserting and verifying whether an element belongs to a set using the Bloom — O(1) filter).
* It can be very useful to prevent unnecessary execution of computationally intensive tasks.,
* just checking that the element is definitely not included in the set.
* It is important to understand that the Bloom filter is a probabilistic data structure: it can tell you with 100% probability.,
* that an element is missing from the dataset, but it is 100% likely to be said.,
* * that the element is in the set, it cannot (false positive results are possible).
*/

pub struct BloomFilter {
    data: Vec<u8>,
    hash_count: u8,
    false_positive_probability: f64,
}

impl BloomFilter {
    /*
     * n - storage size in bytes
     * m - the number of expected items in the storage
     * k - number of hash functions
     */
    pub fn build(n: usize, m: usize, k: u8) -> Self {
        BloomFilter {
            data: vec![0; n],
            hash_count: k,
            false_positive_probability: (1.0
                - f64::exp(-1.0 * k as f64 * m as f64 / (8.0 * n as f64)))
            .powf(k as f64),
        }
    }

    pub fn insert(&mut self, key: &str) {
        for idx in 0..self.hash_count {
            let hash = self.hash(&format!("{}{}", idx, key));
            let mask = 128 >> (hash % 8);
            self.data[hash / 8] |= mask
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        for idx in 0..self.hash_count {
            let hash = self.hash(&format!("{}{}", idx, key));
            let mask = 128 >> (hash % 8);
            if self.data[hash / 8] & mask != mask {
                return false;
            }
        }
        true
    }

    fn hash(&self, s: &str) -> usize {
        let mut hash = 5381_usize;
        for x in s.as_bytes() {
            hash = hash
                .wrapping_shl(5)
                .wrapping_add(hash)
                .wrapping_add(*x as usize);
        }
        hash % (8 * self.data.len())
    }

    pub fn false_positive_probability(&self) -> f64 {
        self.false_positive_probability
    }
}

#[cfg(test)]
#[test]
fn bloom_filter_test() {
    let n = 8 * 1024 * 1024;
    let m = 1000000;
    let k = 2;
    let mut bloom_filter = BloomFilter::build(n, m, k);
    // println!(
    //     "The probability of a false positive result {:.6}",
    //     bloom_filter.false_positive_probability()
    // );

    bloom_filter.insert("google");
    bloom_filter.insert("facebook");
    bloom_filter.insert("yandex");

    assert_eq!(bloom_filter.contains("google"), true);
    assert_eq!(bloom_filter.contains("facebook"), true);
    assert_eq!(bloom_filter.contains("yandex"), true);
    assert_eq!(bloom_filter.contains("microsoft"), false);
    assert_eq!(bloom_filter.contains("oracle"), false);
    assert_eq!(bloom_filter.contains("redhat"), false);
}
