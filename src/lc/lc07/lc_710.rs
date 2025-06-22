// https://leetcode.com/problems/random-pick-with-blacklist/
// 710. Random Pick with Blacklist
use rand::{self, Rng};
pub struct Solution {
    n: i32,
    blacklist: Vec<i32>,
    rng: rand::rngs::ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let mut blacklist = blacklist;
        blacklist.sort();
        blacklist = blacklist
            .into_iter()
            .enumerate()
            .map(|(i, v)| v - i as i32 - 1)
            .collect::<Vec<_>>();
        Self {
            n,
            blacklist,
            rng: rand::thread_rng(),
        }
    }

    pub fn pick(&mut self) -> i32 {
        let r = self.rng.gen_range(0..(self.n - self.blacklist.len() as i32));
        r + self.blacklist.partition_point(|&x| x < r) as i32
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(n, blacklist);
 * let ret_1: i32 = obj.pick();
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_black_list() {
        let mut obj = Solution::new(7, vec![2, 3, 5]);
        for _ in 0..10 {
            assert_ne!(obj.pick(), 2);
            assert_ne!(obj.pick(), 3);
            assert_ne!(obj.pick(), 5);
        }
    }
}
