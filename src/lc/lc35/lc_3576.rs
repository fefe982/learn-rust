// https://leetcode.com/problems/transform-array-to-all-equal-elements/
// 3576. Transform Array to All Equal Elements
pub struct Solution;
impl Solution {
    pub fn can_make_equal(nums: Vec<i32>, k: i32) -> bool {
        let mut mulp = 1;
        let mut muln = 1;
        let mut cntp = 0;
        let mut cntn = 0;
        for i in 0..nums.len() - 1 {
            if nums[i] * mulp > 0 {
                mulp = 1;
            } else {
                mulp = -1;
                cntp += 1;
            }
            if (nums[i] * muln) < 0 {
                muln = 1;
            } else {
                muln = -1;
                cntn += 1;
            }
        }
        let l = nums[nums.len() - 1];
        if (l * mulp > 0 && cntp <= k) || (l * muln < 0 && cntn <= k) {
            true
        } else {
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_make_equal() {
        assert_eq!(Solution::can_make_equal(vec![1, -1, 1, -1, 1], 3), true);
        assert_eq!(Solution::can_make_equal(vec![-1, -1, -1, 1, 1, 1], 5), false);
    }
}
