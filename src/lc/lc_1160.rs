// https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/
// 1160. Find Words That Can Be Formed by Characters
pub struct Solution;
impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut cnt = vec![0; 26];
        for c in chars.chars() {
            cnt[c as usize - 'a' as usize] += 1;
        }
        words.into_iter().fold(0, |acc, word| {
            let mut cc = cnt.clone();
            for c in word.chars() {
                cc[c as usize - 'a' as usize] -= 1;
                if cc[c as usize - 'a' as usize] < 0 {
                    return acc;
                }
            }
            acc + word.len() as i32
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_characters() {
        assert_eq!(
            Solution::count_characters(vec_str!["cat", "bt", "hat", "tree"], "atach".to_string()),
            6
        );
        assert_eq!(
            Solution::count_characters(vec_str!["hello", "world", "leetcode"], "welldonehoneyr".to_string()),
            10
        );
    }
}
