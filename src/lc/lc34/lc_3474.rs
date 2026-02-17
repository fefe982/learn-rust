// https://leetcode.com/problems/lexicographically-smallest-generated-string/
// 3474. Lexicographically Smallest Generated String
pub struct Solution;
impl Solution {
    pub fn generate_string(str1: String, str2: String) -> String {
        let s1 = str1.as_bytes();
        let s2 = str2.chars().collect::<Vec<char>>();
        let mut ans = vec!['.'; s1.len() + s2.len() - 1];
        for i in 0..s1.len() {
            if s1[i] == b'T' {
                for j in 0..s2.len() {
                    if ans[i + j] == '.' {
                        ans[i + j] = s2[j];
                    } else if ans[i + j] != s2[j] {
                        return "".to_string();
                    }
                }
            }
        }
        for i in 0..s1.len() {
            if s1[i] == b'T' {
                continue;
            }
            let mut diff = false;
            let mut tofill = usize::MAX;
            for j in 0..s2.len() {
                if (ans[i + j] == '.' && s2[j] != 'a') || (ans[i + j] != '.' && ans[i + j] != s2[j]) {
                    diff = true;
                    break;
                } else if ans[i + j] == '.' && s2[j] == 'a' {
                    tofill = i + j;
                }
            }
            if !diff {
                if tofill != usize::MAX {
                    ans[tofill] = 'b';
                } else {
                    return "".to_string();
                }
            }
        }
        ans.into_iter()
            .map(|c| if c == '.' { 'a' } else { c })
            .collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_string() {
        assert_eq!(Solution::generate_string("FFTFF".to_string(), "a".to_string()), "bbabb");
        assert_eq!(Solution::generate_string("TFTF".to_string(), "ab".to_string()), "ababa");
        assert_eq!(Solution::generate_string("TFTF".to_string(), "abc".to_string()), "");
        assert_eq!(Solution::generate_string("F".to_string(), "d".to_string()), "a");
    }
}
