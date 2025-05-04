// https://leetcode.com/problems/random-pick-with-weight
// 528. Random Pick with Weight
use rand::distributions::Distribution;
pub struct Solution {
    pub rng: rand::distributions::WeightedIndex<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(w: Vec<i32>) -> Self {
        Self {
            rng: rand::distributions::WeightedIndex::new(w).unwrap(),
        }
    }

    pub fn pick_index(&self) -> i32 {
        self.rng.sample(&mut rand::thread_rng()) as i32
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */
#[cfg(test)]
mod tests {
    use super::*;
    fn check(v: Vec<i32>, n: i32) {
        let obj = Solution::new(v.clone());
        for _ in 0..n {
            assert!(obj.pick_index() < v.len() as i32);
        }
    }
    #[test]
    fn test_528() {
        check(vec![1], 10);
        check(vec![1, 3], 10);
    }
}
