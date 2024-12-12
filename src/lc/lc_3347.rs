// https://leetcode.com/problems/maximum-frequency-of-an-element-after-performing-operations-ii/
// 3347. Maximum Frequency of an Element After Performing Operations II
pub struct Solution;
impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut res = 0;
        let mut l = 0;
        let mut bk = 0;
        let mut bm = 0;
        let mut m = 0;
        let mut r = 0;
        let mut anchor;
        while bk < nums.len() {
            if bm < nums.len() {
                if nums[bk] - k < nums[bm] {
                    anchor = nums[bk];
                    bk += 1;
                } else if nums[bk] - k == nums[bm] {
                    anchor = nums[bk];
                    bk += 1;
                    bm += 1;
                } else {
                    anchor = nums[bm] + k;
                    bm += 1
                }
            } else {
                anchor = nums[bk];
                bk += 1;
            }
            while m < nums.len() && nums[m] < anchor {
                m += 1;
            }
            let mut cnt = 0;
            while m < nums.len() && nums[m] == anchor {
                cnt += 1;
                m += 1;
            }
            while nums[l] < anchor - k {
                l += 1;
            }
            while r < nums.len() && nums[r] <= anchor + k {
                r += 1;
            }
            res = res.max(num_operations.min((r - l - cnt) as i32) + cnt as i32);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_frequency() {
        assert_eq!(Solution::max_frequency(vec![5, 64], 42, 2), 2);
        assert_eq!(Solution::max_frequency(vec![1, 4, 5], 1, 2), 2);
        assert_eq!(Solution::max_frequency(vec![5, 11, 20, 20], 5, 1), 2);
    }
}
