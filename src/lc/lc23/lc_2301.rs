// https://leetcode.com/problems/match-substring-after-replacement/
// 2301. Match Substring After Replacement
pub struct Solution;
impl Solution {
    pub fn match_replacement(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool {
        let mut map = vec![vec![0; 256]; 256];
        for m in mappings {
            map[m[0] as usize][m[1] as usize] = 1;
        }
        let s = s.as_bytes();
        let sub = sub.as_bytes();
        'mtch: for i in 0..=s.len() - sub.len() {
            for j in 0..sub.len() {
                if s[i + j] != sub[j] && map[sub[j] as usize][s[i + j] as usize] == 0 {
                    continue 'mtch;
                }
            }
            return true;
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_match_replacement() {
        assert_eq!(
            Solution::match_replacement(
                "fool3e7bar".to_string(),
                "leet".to_string(),
                vec_vec_chr![["e", "3"], ["t", "7"], ["t", "8"]]
            ),
            true
        );
        assert_eq!(
            Solution::match_replacement("fooleetbar".to_string(), "f00l".to_string(), vec_vec_chr![["o", "0"]]),
            false
        );
        assert_eq!(
            Solution::match_replacement(
                "Fool33tbaR".to_string(),
                "leetd".to_string(),
                vec_vec_chr![["e", "3"], ["t", "7"], ["t", "8"], ["d", "b"], ["p", "b"]]
            ),
            true
        );
    }
}
