// https://leetcode.com/problems/count-unique-characters-of-all-substrings-of-a-given-string/
// 828. Count Unique Characters of All Substrings of a Given String
pub struct Solution;
impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let mut count_one = vec![0; 26];
        let mut count_multi = vec![0; 26];
        let mut count = 0;
        let mut count_one_sum = 0;
        for (i, c) in s.chars().enumerate() {
            let n = (c as i32 - 'A' as i32) as usize;
            let none = i - count_multi[n] - count_one[n];
            count_one_sum = count_one_sum + none + 1 - count_one[n];
            count_multi[n] += count_one[n];
            count_one[n] = none + 1;
            count += count_one_sum;
        }
        count as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_unique_letter_string() {
        assert_eq!(Solution::unique_letter_string("ABC".to_string()), 10);
        assert_eq!(Solution::unique_letter_string("ABA".to_string()), 8);
        assert_eq!(Solution::unique_letter_string("LEETCODE".to_string()), 92);
    }
}
