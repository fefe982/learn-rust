// https://leetcode.com/problems/find-the-score-of-all-prefixes-of-an-array/
// 2640. Find the Score of All Prefixes of an Array
pub struct Solution;
impl Solution {
    pub fn find_prefix_score(nums: Vec<i32>) -> Vec<i64> {
        let mut res = Vec::with_capacity(nums.len());
        let mut max = 0;
        let mut sum = 0;
        for i in 0..nums.len() {
            let mi = nums[i] as i64;
            max = max.max(mi);
            sum += mi + max;
            res.push(sum);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_prefix_score() {
        assert_eq!(
            Solution::find_prefix_score(vec![2, 3, 7, 5, 10]),
            vec![4, 10, 24, 36, 56]
        );
        assert_eq!(
            Solution::find_prefix_score(vec![1, 1, 2, 4, 8, 16]),
            vec![2, 4, 8, 16, 32, 64]
        );
    }
}
