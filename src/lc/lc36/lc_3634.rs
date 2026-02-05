// https://leetcode.com/problems/minimum-removals-to-balance-array/
// 3634. Minimum Removals to Balance Array
pub struct Solution;
impl Solution {
    pub fn min_removal(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let k = k as i64;
        let mut j = nums.partition_point(|&x| (x as i64) <= nums[0] as i64 * k);
        let mut len = j;
        for i in 1..nums.len() {
            let ik = nums[i] as i64 * k;
            while j < nums.len() && (ik >= nums[j] as i64) {
                j += 1;
            }
            len = len.max(j - i);
            if j == nums.len() {
                break;
            }
        }
        (nums.len() - len) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_removal() {
        assert_eq!(Solution::min_removal(vec![2, 1, 5], 2), 1);
        assert_eq!(Solution::min_removal(vec![1, 6, 2, 9], 3), 2);
        assert_eq!(Solution::min_removal(vec![4, 6], 2), 0);
    }
}
