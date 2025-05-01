// https://leetcode.com/problems/longest-word-in-dictionary-through-deleting/
// 524. Longest Word in Dictionary through Deleting
pub struct Solution;
impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let s = s.as_bytes();
        let mut ans = "".to_string();
        for d in dictionary {
            if d.len() < ans.len() {
                continue;
            }
            let mut i = 0;
            let db = d.as_bytes();
            for &c in s {
                if i < db.len() && db[i] == c {
                    i += 1;
                }
            }
            if i == db.len() && (d.len() > ans.len() || d < ans) {
                ans = d;
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
    fn find_longest_word() {
        assert_eq!(
            Solution::find_longest_word("abpcplea".to_string(), vec_str!["ale", "apple", "monkey", "plea"]),
            "apple"
        );
        assert_eq!(
            Solution::find_longest_word("abpcplea".to_string(), vec_str!["a", "b", "c"]),
            "a"
        );
    }
}
