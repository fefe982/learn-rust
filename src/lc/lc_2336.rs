// https://leetcode.com/problems/smallest-number-in-infinite-set/
// 2336. Smallest Number in Infinite Set
struct SmallestInfiniteSet {
    set: std::collections::BTreeSet<i32>,
    bound: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        SmallestInfiniteSet {
            set: std::collections::BTreeSet::new(),
            bound: 1,
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        if let Some(&v) = self.set.iter().next() {
            self.set.remove(&v);
            v
        } else {
            self.bound += 1;
            self.bound - 1
        }
    }

    fn add_back(&mut self, num: i32) {
        if num == self.bound - 1 {
            self.bound -= 1;
            while self.set.contains(&(self.bound - 1)) {
                self.bound -= 1;
                self.set.remove(&self.bound);
            }
        } else if num < self.bound {
            self.set.insert(num);
        }
    }
}

/*
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smallest_infinite_set() {
        let mut smallest_infinite_set = SmallestInfiniteSet::new();
        smallest_infinite_set.add_back(2); // 2 is already in the set, so no change is made.
        assert_eq!(smallest_infinite_set.pop_smallest(), 1); // return 1, since 1 is the smallest number, and remove it from the set.
        assert_eq!(smallest_infinite_set.pop_smallest(), 2); // return 2, and remove it from the set.
        assert_eq!(smallest_infinite_set.pop_smallest(), 3); // return 3, and remove it from the set.
        smallest_infinite_set.add_back(1); // 1 is added back to the set.
        assert_eq!(smallest_infinite_set.pop_smallest(), 1); // return 1, since 1 was added back to the set and
                                                             // is the smallest number, and remove it from the set.
        assert_eq!(smallest_infinite_set.pop_smallest(), 4); // return 4, and remove it from the set.
        assert_eq!(smallest_infinite_set.pop_smallest(), 5); // return 5, and remove it from the set.
    }
}
