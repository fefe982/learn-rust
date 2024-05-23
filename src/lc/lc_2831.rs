// https://leetcode.com/problems/find-the-longest-equal-subarray/
// 2831. Find the Longest Equal Subarray
pub struct Solution;
impl Solution {
    pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = std::collections::HashMap::<i32, i32>::new();
        let n = nums.len();
        let mut i = 0;
        let mut j = 0;
        let mut max = 0;
        while j < n {
            *cnt.entry(nums[j]).or_default() += 1;
            while j - i + 1 - cnt[&nums[i]] as usize > k as usize {
                *cnt.get_mut(&nums[i]).unwrap() -= 1;
                i += 1;
            }
            max = max.max(*cnt.get(&nums[j]).unwrap());
            j += 1;
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_equal_subarray() {
        assert_eq!(Solution::longest_equal_subarray(vec![3, 1, 1], 2), 2);
        assert_eq!(Solution::longest_equal_subarray(vec![1, 3, 2, 3, 1, 3], 3), 3);
        assert_eq!(Solution::longest_equal_subarray(vec![1, 1, 2, 2, 1, 1], 2), 4);
    }
}
