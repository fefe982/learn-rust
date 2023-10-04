// https://leetcode.com/problems/design-hashmap/
// 706. Design HashMap
#[allow(dead_code)]
const BUCKET: i32 = 997;

#[allow(dead_code)]
struct MyHashMap {
    buckets: Vec<Vec<i32>>,
    vals: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); BUCKET as usize],
            vals: vec![Vec::new(); BUCKET as usize],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let nbucket = (key % BUCKET) as usize;
        match self.buckets[nbucket].binary_search(&key) {
            Err(pos) => {
                self.buckets[nbucket].insert(pos, key);
                self.vals[nbucket].insert(pos, value);
            }
            Ok(pos) => {
                self.vals[nbucket][pos] = value;
            }
        }
    }

    fn get(&self, key: i32) -> i32 {
        let nbucket = (key % BUCKET) as usize;
        match self.buckets[nbucket].binary_search(&key) {
            Ok(pos) => self.vals[nbucket][pos],
            Err(_) => -1,
        }
    }

    fn remove(&mut self, key: i32) {
        let nbucket = (key % BUCKET) as usize;
        if let Ok(pos) = self.buckets[nbucket].binary_search(&key) {
            self.buckets[nbucket].remove(pos);
            self.vals[nbucket].remove(pos);
        }
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash_map() {
        let mut s = MyHashMap::new();
        s.put(1, 1);
        s.put(2, 2);
        assert_eq!(s.get(1), 1);
        assert_eq!(s.get(3), -1);
        s.put(2, 1);
        assert_eq!(s.get(2), 1);
        s.remove(2);
        assert_eq!(s.get(2), -1);
    }
}
