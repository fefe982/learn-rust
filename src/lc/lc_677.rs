// https://leetcode.com/problems/map-sum-pairs/
// 677. Map Sum Pairs

struct MapSum {
    leaf: i32,
    sum: i32,
    children: std::collections::HashMap<u8, MapSum>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {
    fn new() -> Self {
        Self {
            leaf: 0,
            sum: 0,
            children: std::collections::HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        let val = val - self.common_prefix_search(key.as_bytes()).1;
        self.insert_u8(key.as_bytes(), val)
    }

    fn sum(&self, prefix: String) -> i32 {
        self.common_prefix_search(prefix.as_bytes()).0
    }

    fn insert_u8(&mut self, s: &[u8], val: i32) {
        self.sum += val;
        if s.len() != 0 {
            self.children
                .entry(s[0])
                .or_insert_with(|| MapSum::new())
                .insert_u8(&s[1..], val);
        } else {
            self.leaf += val;
        }
    }
    fn common_prefix_search(&self, s: &[u8]) -> (i32, i32) {
        if s.len() > 0 {
            if let Some(child) = self.children.get(&s[0]) {
                child.common_prefix_search(&s[1..])
            } else {
                (0, 0)
            }
        } else {
            (self.sum, self.leaf)
        }
    }
}

/**
 * Your MapSum object will be instantiated and called as such:
 * let obj = MapSum::new();
 * obj.insert(key, val);
 * let ret_2: i32 = obj.sum(prefix);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut obj = MapSum::new();
        obj.insert("apple".to_string(), 3);
        assert_eq!(obj.sum("ap".to_string()), 3);
        obj.insert("app".to_string(), 2);
        obj.insert("apple".to_string(), 2);
        assert_eq!(obj.sum("ap".to_string()), 4);
    }
}
