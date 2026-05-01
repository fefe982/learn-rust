// https://leetcode.com/problems/count-number-of-pairs-with-absolute-difference-k/
// 2006. Count Number of Pairs With Absolute Difference K
pub struct Solution;
impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut count = HashMap::new();
        let mut ans = 0;
        for &num in &nums {
            ans += count.get(&(num - k)).unwrap_or(&0);
            ans += count.get(&(num + k)).unwrap_or(&0);
            *count.entry(num).or_insert(0) += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_k_difference() {
        assert_eq!(Solution::count_k_difference(vec![1, 2, 2, 1], 1), 4);
        assert_eq!(Solution::count_k_difference(vec![1, 3], 3), 0);
        assert_eq!(Solution::count_k_difference(vec![3, 2, 1, 5, 4], 2), 3);
    }
}
