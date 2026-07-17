// https://leetcode.com/problems/ways-to-split-array-into-good-subarrays/
// 2750. Ways to Split Array into Good Subarrays
pub struct Solution;
impl Solution {
    pub fn number_of_good_subarray_splits(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut i = 0;
        while i < nums.len() && nums[i] == 0 {
            i += 1;
        }
        if i < nums.len() && nums[i] == 1 {
            i += 1;
        } else {
            return 0;
        }
        let mut r = 1;
        while i < nums.len() {
            let mut seg = 0;
            while i < nums.len() && nums[i] == 0 {
                i += 1;
                seg += 1;
            }
            if i < nums.len() && nums[i] == 1 {
                i += 1;
                r = (r * (seg + 1)) % MOD;
            }
        }
        r as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_good_subarray_splits() {
        assert_eq!(Solution::number_of_good_subarray_splits(vec![0, 1, 0, 0, 1]), 3);
        assert_eq!(Solution::number_of_good_subarray_splits(vec![0, 1, 0]), 1);
    }
}
