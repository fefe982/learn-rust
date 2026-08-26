// https://leetcode.com/problems/lexicographically-smallest-permutation-greater-than-target/
// 3720. Lexicographically Smallest Permutation Greater Than Target
pub struct Solution;
impl Solution {
    pub fn lex_greater_permutation(s: String, target: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let target = target.chars().collect::<Vec<char>>();
        let mut cnt = vec![0; 26];
        for &c in &s {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
        let mut i = 0;
        'i: while i <= s.len() {
            if i < s.len() && cnt[(target[i] as u8 - b'a') as usize] > 0 {
                s[i] = target[i];
                cnt[(target[i] as u8 - b'a') as usize] -= 1;
                i += 1;
            } else {
                if i == s.len() {
                    i -= 1;
                    cnt[(target[i] as u8 - b'a') as usize] += 1;
                }
                let mut idxt = (target[i] as u8 - b'a') as usize;
                loop {
                    for j in idxt + 1..26 {
                        if cnt[j] > 0 {
                            s[i] = (j as u8 + b'a') as char;
                            cnt[j] -= 1;
                            i += 1;
                            break 'i;
                        }
                    }
                    if i == 0 {
                        return "".to_string();
                    }
                    i -= 1;
                    idxt = (target[i] as u8 - b'a') as usize;
                    cnt[idxt] += 1;
                }
            }
        }
        let mut ic = 0;
        while i < s.len() {
            while cnt[ic] == 0 {
                ic += 1;
            }
            s[i] = (ic as u8 + b'a') as char;
            cnt[ic] -= 1;
            i += 1;
        }
        s.into_iter().collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lex_greater_permutation() {
        assert_eq!(
            Solution::lex_greater_permutation("ab".to_string(), "ab".to_string()),
            "ba"
        );
        assert_eq!(
            Solution::lex_greater_permutation("abc".to_string(), "bba".to_string()),
            "bca"
        );
        assert_eq!(
            Solution::lex_greater_permutation("leet".to_string(), "code".to_string()),
            "eelt"
        );
        assert_eq!(
            Solution::lex_greater_permutation("baba".to_string(), "bbaa".to_string()),
            ""
        );
    }
}
