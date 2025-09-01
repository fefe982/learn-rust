// https://leetcode.com/problems/find-the-number-of-ways-to-place-people-i/
// 3025. Find the Number of Ways to Place People I
pub struct Solution;
impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        super::lc_3027::Solution::number_of_pairs(points)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn number_of_pairs() {
        assert_eq!(Solution::number_of_pairs(vec_vec![[1, 1], [2, 2], [3, 3]]), 0);
        assert_eq!(Solution::number_of_pairs(vec_vec![[3, 1], [1, 3], [1, 1]]), 2);
    }
}
