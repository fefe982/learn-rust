// https://leetcode.com/problems/maximum-score-from-removing-stones/
// 1753. Maximum Score From Removing Stones
pub struct Solution;
impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let sum = a + b + c;
        let max = a.max(b).max(c);
        (sum - max).min(sum / 2)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_score() {
        assert_eq!(Solution::maximum_score(2, 4, 6), 6);
        assert_eq!(Solution::maximum_score(4, 4, 6), 7);
        assert_eq!(Solution::maximum_score(1, 8, 8), 8);
    }
}
