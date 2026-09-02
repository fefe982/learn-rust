// https://leetcode.com/problems/reach-end-of-array-with-max-score/
// 3282. Reach End of Array With Max Score
pub struct Solution;
impl Solution {
    pub fn find_maximum_score(nums: Vec<i32>) -> i64 {
        let mut m = nums[0];
        let mut s = 0;
        for i in 1..nums.len() {
            s += m as i64;
            m = m.max(nums[i]);
        }
        s
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_maximum_score() {
        assert_eq!(Solution::find_maximum_score(vec![1, 3, 1, 5]), 7);
        assert_eq!(Solution::find_maximum_score(vec![4, 3, 1, 3, 2]), 16);
    }
}
