// https://leetcode.com/problems/lfu-cache/
// 460. LFU Cache
#[derive(Copy, Clone)]
struct Node {
    key: i32,
    val: i32,
    cnt: i32,
    prev: usize,
    next: usize,
}
pub struct LFUCache {
    nodes: Vec<Node>,
    sz: usize,
    map: std::collections::HashMap<i32, usize>,
    ins: std::collections::HashMap<i32, usize>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        let mut s = Self {
            nodes: vec![
                Node {
                    key: 0,
                    val: 0,
                    cnt: 0,
                    prev: usize::MAX,
                    next: usize::MAX
                };
                capacity as usize + 2
            ],
            sz: 2,
            map: std::collections::HashMap::new(),
            ins: std::collections::HashMap::new(),
        };
        s.nodes[0].next = 1;
        s.nodes[1].prev = 0;
        s.nodes[1].cnt = i32::MAX;
        s
    }
    fn remove(&mut self, idx: usize) {
        let prev = self.nodes[idx].prev;
        let next = self.nodes[idx].next;
        self.nodes[prev].next = next;
        self.nodes[next].prev = prev;
    }
    fn insert(&mut self, idx: usize, prev: usize) {
        let next = self.nodes[prev].next;
        self.nodes[prev].next = idx;
        self.nodes[idx].prev = prev;
        self.nodes[next].prev = idx;
        self.nodes[idx].next = next;
    }
    fn get_idx(&mut self, key: i32) -> usize {
        if let Some(&idx) = self.map.get(&key) {
            let cnt = self.nodes[idx].cnt;
            let prev = self.nodes[idx].prev;
            let mut ins = prev;
            let ins_cnt = *self.ins.get(&cnt).unwrap();
            if ins_cnt == idx {
                if self.nodes[prev].cnt == cnt {
                    self.ins.insert(cnt, prev);
                } else {
                    self.ins.remove(&cnt);
                }
            } else {
                ins = ins_cnt;
            }
            self.nodes[idx].cnt += 1;
            if let Some(&ins_cnt_1) = self.ins.get(&(cnt + 1)) {
                ins = ins_cnt_1;
            }
            if ins != prev {
                self.remove(idx);
                self.insert(idx, ins);
            }
            self.ins.insert(cnt + 1, idx);
            idx
        } else {
            usize::MAX
        }
    }
    pub fn get(&mut self, key: i32) -> i32 {
        let idx = self.get_idx(key);
        if idx == usize::MAX {
            -1
        } else {
            self.nodes[idx].val
        }
    }
    pub fn put(&mut self, key: i32, value: i32) {
        let idx = self.get_idx(key);
        if idx != usize::MAX {
            self.nodes[idx].val = value;
        } else {
            let idx = if self.sz < self.nodes.len() {
                self.sz += 1;
                self.sz - 1
            } else {
                let i = self.nodes[0].next;
                self.remove(i);
                self.map.remove(&self.nodes[i].key);
                if *self.ins.get(&self.nodes[i].cnt).unwrap() == i {
                    self.ins.remove(&self.nodes[i].cnt);
                }
                i
            };
            self.nodes[idx].key = key;
            self.nodes[idx].val = value;
            self.nodes[idx].cnt = 1;
            let &prev = self.ins.get(&1).unwrap_or(&0);
            self.insert(idx, prev);
            self.map.insert(key, idx);
            self.ins.insert(1, idx);
        }
    }
}

/**
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lru_cache() {
        let mut obj = LFUCache::new(2);
        obj.put(1, 1);
        obj.put(2, 2);
        assert_eq!(obj.get(1), 1);
        obj.put(3, 3);
        assert_eq!(obj.get(2), -1);
        assert_eq!(obj.get(3), 3);
        obj.put(4, 4);
        assert_eq!(obj.get(1), -1);
        assert_eq!(obj.get(3), 3);
        assert_eq!(obj.get(4), 4);
    }
}
