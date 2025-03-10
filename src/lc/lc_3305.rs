// https://leetcode.com/problems/count-of-substrings-containing-every-vowel-and-k-consonants-i/
// 3305. Count of Substrings Containing All Vowels and K Consonants I
pub struct Solution;
impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i32 {
        super::lc_3306::Solution::count_of_substrings(word, k) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_of_substrings() {
        assert_eq!(Solution::count_of_substrings("aeioqq".to_string(), 12), 0);
        assert_eq!(Solution::count_of_substrings("aeiou".to_string(), 0), 1);
        assert_eq!(Solution::count_of_substrings("ieaouqqieaouqq".to_string(), 1), 3);
    }
}
