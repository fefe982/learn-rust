// https://leetcode.com/problems/minimum-number-of-operations-to-make-arrays-similar/
// 2449. Minimum Number of Operations to Make Arrays Similar
pub struct Solution;
impl Solution {
    pub fn make_similar(nums: Vec<i32>, target: Vec<i32>) -> i64 {
        let mut nums = nums;
        let mut target = target;
        nums.sort_unstable();
        target.sort_unstable();
        let mut oi = 0;
        let mut ei = 0;
        while oi < nums.len() && nums[oi] % 2 == 0 {
            oi += 1;
        }
        while ei < nums.len() && nums[ei] % 2 == 1 {
            ei += 1;
        }
        let mut ans = 0;
        for i in 0..target.len() {
            if target[i] % 2 == 0 {
                ans += (nums[ei] - target[i]).max(0) as i64 / 2;
                ei += 1;
                while ei < nums.len() && nums[ei] % 2 == 1 {
                    ei += 1;
                }
            } else {
                ans += (nums[oi] - target[i]).max(0) as i64 / 2;
                oi += 1;
                while oi < nums.len() && nums[oi] % 2 == 0 {
                    oi += 1;
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn make_similar() {
        assert_eq!(Solution::make_similar(vec![8, 12, 6], vec![2, 14, 10]), 2);
        assert_eq!(Solution::make_similar(vec![1, 2, 5], vec![4, 1, 3]), 1);
        assert_eq!(Solution::make_similar(vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]), 0);
    }
}
