// https://leetcode.com/problems/subarrays-distinct-element-sum-of-squares-i/
// 2913. Subarrays Distinct Element Sum of Squares I
pub struct Solution;
impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let mut s = 0;
        for i in 0..nums.len() {
            let mut set = std::collections::HashSet::new();
            for j in i..nums.len() {
                set.insert(nums[j]);
                s += set.len() * set.len();
            }
        }
        s as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_counts() {
        assert_eq!(Solution::sum_counts(vec![1, 2, 1]), 15);
        assert_eq!(Solution::sum_counts(vec![1, 1]), 3);
    }
}
