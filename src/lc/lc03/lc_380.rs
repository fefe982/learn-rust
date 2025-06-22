// https://leetcode.com/problems/insert-delete-getrandom-o1/description/
// 380. Insert Delete GetRandom O(1)
pub struct RandomizedSet {
    v: Vec<i32>,
    m: std::collections::HashMap<i32, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    pub fn new() -> Self {
        Self {
            v: vec![],
            m: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        if self.m.get(&val).is_none() {
            self.m.insert(val, self.v.len());
            self.v.push(val);
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, val: i32) -> bool {
        if let Some(&idx) = self.m.get(&val) {
            if idx + 1 != self.v.len() {
                let l = self.v.len();
                self.v.swap(idx, l - 1);
                self.m.insert(self.v[idx], idx);
            }
            self.m.remove(&val);
            self.v.pop();
            true
        } else {
            false
        }
    }

    pub fn get_random(&self) -> i32 {
        self.v[rand::random::<usize>() % self.v.len()]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_randomized_set() {
        let mut obj = RandomizedSet::new();
        assert_eq!(obj.insert(1), true);
        assert_eq!(obj.remove(2), false);
        assert_eq!(obj.insert(2), true);
        obj.get_random();
        assert_eq!(obj.remove(1), true);
        assert_eq!(obj.remove(2), true);
        assert_eq!(obj.insert(2), true);
    }
}
