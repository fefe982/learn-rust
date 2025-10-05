// https://leetcode.com/problems/minimum-subsequence-in-non-increasing-order/
// 1403. Minimum Subsequence in Non-Increasing Order
pub struct Solution;
impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| b.cmp(a));
        let mut sum = nums.iter().sum::<i32>();
        let mut ans = Vec::new();
        let mut ans_sum = 0;
        for num in nums {
            ans.push(num);
            ans_sum += num;
            sum -= num;
            if ans_sum > sum {
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
    fn min_subsequence() {
        assert_eq!(Solution::min_subsequence(vec![4, 3, 10, 9, 8]), vec![10, 9]);
        assert_eq!(Solution::min_subsequence(vec![4, 4, 7, 6, 7]), vec![7, 7, 6]);
    }
}
