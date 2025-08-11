// https://leetcode.com/problems/find-maximum-number-of-string-pairs/
// 2744. Find Maximum Number of String Pairs
pub struct Solution;
impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut ans = 0;
        let mut set = std::collections::HashSet::new();
        for w in words {
            if set.contains(&w) {
                ans += 1;
            } else {
                let rw = w.chars().rev().collect::<String>();
                set.insert(rw);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_maximum_number_of_string_pairs() {
        assert_eq!(
            Solution::maximum_number_of_string_pairs(vec_str!["cd", "ac", "dc", "ca", "zz"]),
            2
        );
        assert_eq!(Solution::maximum_number_of_string_pairs(vec_str!["ab", "ba", "cc"]), 1);
        assert_eq!(Solution::maximum_number_of_string_pairs(vec_str!["aa", "ab"]), 0);
    }
}
