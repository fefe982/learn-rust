// https://leetcode.com/problems/number-of-unequal-triplets-in-array/
// 2475. Number of Unequal Triplets in Array
pub struct Solution;
impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let l = nums.len() as i32;
        let mut cnt = l * (l - 1) * (l - 2) / 6;
        let mut s = std::collections::HashMap::<i32, i32>::new();
        for n in nums {
            *s.entry(n).or_default() += 1;
        }
        for (_, c) in s {
            if c >= 2 {
                cnt -= c * (c - 1) / 2 * (l - c);
            }
            if c >= 3 {
                cnt -= c * (c - 1) * (c - 2) / 6;
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unequal_triplets() {
        assert_eq!(Solution::unequal_triplets(vec![4, 4, 2, 4, 3]), 3);
        assert_eq!(Solution::unequal_triplets(vec![1, 1, 1, 1, 1]), 0);
    }
}
