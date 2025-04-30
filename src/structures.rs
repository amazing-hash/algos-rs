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
