// https://leetcode.com/problems/lru-cache/
// 146. LRU Cache
#[derive(Copy, Clone)]
struct Node {
    key: i32,
    val: i32,
    prev: usize,
    next: usize,
}
pub struct LRUCache {
    nodes: Vec<Node>,
    sz: usize,
    map: std::collections::BTreeMap<i32, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let mut s = Self {
            nodes: vec![
                Node {
                    key: 0,
                    val: 0,
                    prev: usize::MAX,
                    next: usize::MAX
                };
                capacity as usize + 2
            ],
            sz: 2,
            map: std::collections::BTreeMap::new(),
        };
        s.nodes[0].next = 1;
        s.nodes[1].prev = 0;
        s
    }

    fn remove(&mut self, idx: usize) {
        let prev = self.nodes[idx].prev;
        let next = self.nodes[idx].next;
        self.nodes[prev].next = next;
        self.nodes[next].prev = prev;
    }
    fn insert(&mut self, idx: usize) {
        let prev = self.nodes[1].prev;
        self.nodes[prev].next = idx;
        self.nodes[idx].prev = prev;
        self.nodes[1].prev = idx;
        self.nodes[idx].next = 1;
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&idx) = self.map.get(&key) {
            if idx != self.nodes[1].prev {
                self.remove(idx);
                self.insert(idx);
            }
            self.nodes[idx].val
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&idx) = self.map.get(&key) {
            if idx != self.nodes[1].prev {
                self.remove(idx);
                self.insert(idx);
            }
            self.nodes[idx].val = value;
        } else {
            let idx = if self.sz < self.nodes.len() {
                self.sz += 1;
                self.sz - 1
            } else {
                let i = self.nodes[0].next;
                self.remove(i);
                self.map.remove(&self.nodes[i].key);
                i
            };
            self.nodes[idx].key = key;
            self.nodes[idx].val = value;
            self.insert(idx);
            self.map.insert(key, idx);
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lru_cache() {
        let mut obj = LRUCache::new(2);
        obj.put(1, 1);
        obj.put(2, 2);
        assert_eq!(obj.get(1), 1);
        obj.put(3, 3);
        assert_eq!(obj.get(2), -1);
        obj.put(4, 4);
        assert_eq!(obj.get(1), -1);
        assert_eq!(obj.get(3), 3);
        assert_eq!(obj.get(4), 4);
    }
}
