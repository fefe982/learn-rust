// https://leetcode.com/problems/design-hashset/
// 705. Design HashSet
const BUCKET: i32 = 997;
struct MyHashSet {
    buckets: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); BUCKET as usize],
        }
    }

    fn add(&mut self, key: i32) {
        let nbucket = (key % BUCKET) as usize;
        if let Err(pos) = self.buckets[nbucket].binary_search(&key) {
            self.buckets[nbucket].insert(pos, key);
        }
    }

    fn remove(&mut self, key: i32) {
        let nbucket = (key % BUCKET) as usize;
        if let Ok(pos) = self.buckets[nbucket].binary_search(&key) {
            self.buckets[nbucket].remove(pos);
        }
    }

    fn contains(&self, key: i32) -> bool {
        let nbucket = (key % BUCKET) as usize;
        match self.buckets[nbucket].binary_search(&key) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hash_set() {
        let mut s = MyHashSet::new();
        s.add(1);
        s.add(2);
        assert_eq!(s.contains(1), true);
        assert_eq!(s.contains(3), false);
        s.add(2);
        assert_eq!(s.contains(2), true);
        s.remove(2);
        assert_eq!(s.contains(2), false);
    }
}
