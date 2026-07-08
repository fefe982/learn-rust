// https://leetcode.com/problems/rearrange-array-to-maximize-prefix-score/
// 2587. Rearrange Array to Maximize Prefix Score
pub struct Solution;
impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| b.cmp(a));
        let mut sum = 0;
        let mut ans = 0;
        for num in nums {
            sum += num as i64;
            if sum > 0 {
                ans += 1;
            } else {
                break;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_score() {
        assert_eq!(Solution::max_score(vec![2, -1, 0, 1, -3, 3, -3]), 6);
        assert_eq!(Solution::max_score(vec![-2, -3, 0]), 0);
    }
}
