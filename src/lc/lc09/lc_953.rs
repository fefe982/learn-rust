// https://leetcode.com/problems/verifying-an-alien-dictionary/
// 953. Verifying an Alien Dictionary
pub struct Solution;
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut ord = vec![0; 26];
        for (i, c) in order.bytes().enumerate() {
            ord[(c - b'a') as usize] = i;
        }
        'w: for i in 1..words.len() {
            let w0 = words[i - 1].as_bytes();
            let w1 = words[i].as_bytes();
            for (c0, c1) in w0.iter().zip(w1.iter()) {
                if c0 != c1 {
                    if ord[(c0 - b'a') as usize] > ord[(c1 - b'a') as usize] {
                        return false;
                    } else {
                        continue 'w;
                    }
                }
            }
            if w0.len() > w1.len() {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn is_alien_sorted() {
        assert_eq!(
            Solution::is_alien_sorted(vec_str!["hello", "leetcode"], "hlabcdefgijkmnopqrstuvwxyz".to_string()),
            true
        );
        assert_eq!(
            Solution::is_alien_sorted(
                vec_str!["word", "world", "row"],
                "worldabcefghijkmnpqstuvxyz".to_string()
            ),
            false
        );
        assert_eq!(
            Solution::is_alien_sorted(vec_str!["apple", "app"], "abcdefghijklmnopqrstuvwxyz".to_string()),
            false
        );
    }
}
