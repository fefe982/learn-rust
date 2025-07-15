// https://leetcode.com/problems/number-of-bit-changes-to-make-two-integers-equal/
// 3226. Number of Bit Changes to Make Two Integers Equal
pub struct Solution;
impl Solution {
    pub fn min_changes(n: i32, k: i32) -> i32 {
        if n & k != k {
            return -1;
        }
        (n ^ k).count_ones() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_changes() {
        assert_eq!(Solution::min_changes(13, 4), 2);
        assert_eq!(Solution::min_changes(21, 21), 0);
        assert_eq!(Solution::min_changes(14, 13), -1);
    }
}
