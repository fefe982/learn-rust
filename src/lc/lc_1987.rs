// https://leetcode.com/problems/number-of-unique-good-subsequences/
// 1987. Number of Unique Good Subsequences
pub struct Solution;
impl Solution {
    pub fn number_of_unique_good_subsequences(binary: String) -> i32 {
        let mut has0 = 0;
        let mut end0 = 0;
        let mut end1 = 0;
        let m = 1_0000_0000_7i64;
        for c in binary.bytes() {
            if c == b'0' {
                has0 = 1;
                end0 = (end0 + end1) % m;
            } else {
                end1 = (end1 + end0 + 1) % m;
            }
        }
        ((has0 + end0 + end1) % m) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_unique_good_subsequences() {
        assert_eq!(Solution::number_of_unique_good_subsequences("001".to_string()), 2);
        assert_eq!(Solution::number_of_unique_good_subsequences("11".to_string()), 2);
        assert_eq!(Solution::number_of_unique_good_subsequences("101".to_string()), 5);
    }
}
