// https://leetcode.com/problems/find-the-count-of-monotonic-pairs-i/
// 3250. Find the Count of Monotonic Pairs in an Array I
pub struct Solution;
impl Solution {
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        super::lc_3251::Solution::count_of_pairs(nums)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_of_pairs() {
        assert_eq!(Solution::count_of_pairs(vec![2, 3, 2]), 4);
        assert_eq!(Solution::count_of_pairs(vec![5, 5, 5, 5]), 126);
    }
}
