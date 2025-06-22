// https://leetcode.com/problems/set-mismatch/
// 645. Set Mismatch
pub struct Solution;
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut dup = 1;
        let mut xn = 0;
        let mut xa = 0;
        let mut nums = nums;
        for i in 0..nums.len() {
            xa ^= i as i32 + 1;
            let ni = nums[i].abs();
            xn ^= ni;
            let ii = ni as usize - 1;
            if nums[ii] < 0 {
                dup = ni;
            }
            nums[ii] *= -1;
        }
        vec![dup, xa ^ xn ^ dup]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_error_nums() {
        assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
        assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
    }
}
