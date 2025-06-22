// https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/
// 381. Insert Delete GetRandom O(1) - Duplicates allowed
use rand::{self, Rng};
pub struct RandomizedCollection {
    nums: Vec<(i32, usize)>,
    map: std::collections::HashMap<i32, Vec<usize>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {
    pub fn new() -> Self {
        Self {
            nums: vec![],
            map: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        let v = self.map.entry(val).or_default();
        let vi = v.len();
        let ni = self.nums.len();
        self.nums.push((val, vi));
        v.push(ni);
        vi == 0
    }

    pub fn remove(&mut self, val: i32) -> bool {
        if let Some(v) = self.map.get_mut(&val) {
            let ni = v.pop().unwrap();
            self.nums.swap_remove(ni);
            if v.is_empty() {
                self.map.remove(&val);
            }
            if ni < self.nums.len() {
                self.map.get_mut(&self.nums[ni].0).unwrap()[self.nums[ni].1] = ni;
            }
            true
        } else {
            false
        }
    }

    pub fn get_random(&self) -> i32 {
        let mut rng = rand::rngs::OsRng {};
        self.nums[rng.gen_range(0..self.nums.len())].0
    }
}

/**
 * Your RandomizedCollection object will be instantiated and called as such:
 * let obj = RandomizedCollection::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn random_collection() {
        let mut obj = RandomizedCollection::new();
        assert_eq!(obj.insert(1), true);
        assert_eq!(obj.insert(1), false);
        assert_eq!(obj.insert(2), true);
        obj.get_random();
        assert_eq!(obj.remove(1), true);
        obj.get_random();
    }
}
