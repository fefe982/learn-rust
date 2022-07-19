// https://leetcode.com/problems/two-sum/
// 1. Two Sum

use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = HashMap::<i32, i32>::new();
        for (idx, val) in (0..).zip(nums) {
            match m.get(&val) {
                Some(v) => return vec![*v, idx],
                None => {
                    m.insert(target - val, idx);
                }
            }
        }
        Vec::<i32>::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
    }
}
