// https://leetcode.com/problems/minimum-deletions-to-make-array-beautiful/
// 2216. Minimum Deletions to Make Array Beautiful
pub struct Solution;
impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        let np = nums
            .iter()
            .skip(1)
            .fold((nums[0], (0, 0)), |(prev, (c1, c0)), &n| {
                if n == prev {
                    (n, (c0, c0))
                } else {
                    (n, (c0, c0.max(c1 + 1)))
                }
            })
            .1
             .1;
        nums.len() as i32 - np * 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_deletion() {
        assert_eq!(Solution::min_deletion(vec![1, 1, 2, 3, 5]), 1);
        assert_eq!(Solution::min_deletion(vec![1, 1, 2, 2, 3, 3]), 2);
    }
}
