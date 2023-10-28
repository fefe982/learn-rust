// https://leetcode.com/problems/poor-pigs/
// 458. Poor Pigs
pub struct Solution;
impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        ((buckets as f32).ln() / ((minutes_to_test / minutes_to_die + 1) as f32).ln()).ceil() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn poor_pigs() {
        assert_eq!(Solution::poor_pigs(125, 1, 4), 3);
        assert_eq!(Solution::poor_pigs(4, 15, 15), 2);
        assert_eq!(Solution::poor_pigs(4, 15, 30), 2);
    }
}
