// https://leetcode.com/problems/split-array-with-minimum-difference/
// 3698. Split Array With Minimum Difference
pub struct Solution;
impl Solution {
    pub fn split_array(nums: Vec<i32>) -> i64 {
        let mut l = 0;
        let mut sum_l = nums[0] as i64;
        while l + 1 < nums.len() && nums[l + 1] > nums[l] {
            l += 1;
            sum_l += nums[l] as i64;
        }
        let mut r = nums.len() - 1;
        let mut sum_r = nums[r] as i64;
        while r > 0 && nums[r - 1] > nums[r] {
            r -= 1;
            sum_r += nums[r] as i64;
        }
        if l + 1 == r {
            (sum_l - sum_r).abs()
        } else if l == r {
            let d = sum_l - sum_r;
            let m = nums[l] as i64;
            (d - m).abs().min((d + m).abs())
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn split_array() {
        assert_eq!(Solution::split_array(vec![2, 2]), 0);
        assert_eq!(Solution::split_array(vec![1, 3, 2]), 2);
        assert_eq!(Solution::split_array(vec![1, 2, 4, 3]), 4);
        assert_eq!(Solution::split_array(vec![3, 1, 2]), -1);
    }
}
