// https://leetcode.com/problems/k-diff-pairs-in-an-array
// 532. K-diff Pairs in an Array
pub struct Solution;
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ans = 0;
        let mut i = 0;
        let mut j = 1;
        while j < nums.len() {
            while j + 1 < nums.len() && nums[j + 1] == nums[j] {
                j += 1;
            }
            while nums[i] < nums[j] - k {
                i += 1;
            }
            if i == j {
                j += 1;
                continue;
            }
            if nums[j] - nums[i] == k {
                ans += 1;
            }
            j += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_pairs() {
        assert_eq!(Solution::find_pairs(vec![3, 1, 4, 1, 5], 2), 2);
        assert_eq!(Solution::find_pairs(vec![1, 2, 3, 4, 5], 1), 4);
        assert_eq!(Solution::find_pairs(vec![1, 3, 1, 5, 4], 0), 1);
    }
}
