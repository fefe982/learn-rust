// https://leetcode.com/problems/number-of-squareful-arrays/
// 996. Number of Squareful Arrays
pub struct Solution;
impl Solution {
    fn check(a: i32, b: i32) -> bool {
        let s = ((a + b) as f64).sqrt() as i32;
        return s * s == a + b;
    }
    fn count(last: i32, mask: i32, nums: &Vec<i32>) -> i32 {
        if mask == 0 {
            return 1;
        }
        let mut cnt = 0;
        let mut m = mask;
        let mut l = -1;
        while m > 0 {
            let idx = ((m ^ (m - 1)).trailing_ones() - 1) as usize;
            if nums[idx] != l && (last == -1 || Solution::check(last, nums[idx])) {
                cnt += Self::count(nums[idx], mask & !(1 << idx), nums);
            }
            l = nums[idx];
            m &= m - 1;
        }
        cnt
    }
    pub fn num_squareful_perms(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        Solution::count(-1, (1 << nums.len()) - 1, &nums)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_squareful_perms() {
        assert_eq!(Solution::num_squareful_perms(vec![1, 17, 8]), 2);
        assert_eq!(Solution::num_squareful_perms(vec![2, 2, 2]), 1);
    }
}
