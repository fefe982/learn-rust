// https://leetcode.com/problems/count-sorted-vowel-strings/
// 1641. Count Sorted Vowel Strings
pub struct Solution;
impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        (n + 4)*(n + 3)*(n + 2)*(n + 1) / 24
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_vowel_strings() {
        assert_eq!(Solution::count_vowel_strings(1), 5);
        assert_eq!(Solution::count_vowel_strings(2), 15);
        assert_eq!(Solution::count_vowel_strings(33), 66045);
    }
}
