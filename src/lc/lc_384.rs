// https://leetcode.com/problems/shuffle-an-array/
// 384. Shuffle an Array
use rand::Rng;
pub struct Solution {
    origin: Vec<i32>,
    rng: rand::rngs::ThreadRng,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(nums: Vec<i32>) -> Self {
        Self {
            origin: nums,
            rng: rand::thread_rng(),
        }
    }

    pub fn reset(&self) -> Vec<i32> {
        self.origin.clone()
    }

    pub fn shuffle(&mut self) -> Vec<i32> {
        let mut shuffled = self.origin.clone();
        for i in (1..shuffled.len()).rev() {
            let j = self.rng.gen_range(0..=i);
            shuffled.swap(i, j);
        }
        shuffled
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn shuffle() {
        let mut s = Solution::new(vec![1, 2, 3]);
        assert_sort_eq!(s.shuffle(), vec![1, 2, 3]);
        assert_eq!(s.reset(), vec![1, 2, 3]);
        assert_sort_eq!(s.shuffle(), vec![1, 2, 3]);
    }
}
