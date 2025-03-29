// https://leetcode.com/problems/random-pick-index
// 398. Random Pick Index
use rand::Rng;
pub struct Solution {
    map: std::collections::HashMap<i32, Vec<i32>>,
    rng: rand::rngs::ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut map = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            map.entry(num).or_insert_with(Vec::new).push(i as i32);
        }
        Self {
            map,
            rng: rand::thread_rng(),
        }
    }

    pub fn pick(&mut self, target: i32) -> i32 {
        let indices = self.map.get(&target).unwrap();
        let random_index = self.rng.gen_range(0..indices.len());
        indices[random_index]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_398() {
        let mut obj = Solution::new(vec![1, 2, 3, 3, 3]);
        assert!(vec![2, 3, 4].contains(&obj.pick(3)));
        assert_eq!(obj.pick(1), 0);
        assert!(vec![2, 3, 4].contains(&obj.pick(3)));
    }
}
