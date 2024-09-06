// https://leetcode.com/problems/find-the-string-with-lcp/
// 2573. Find the String With LCP
pub struct Solution;
impl Solution {
    fn check(s: &[u8], z: &[i32]) -> bool {
        let n = s.len();
        let mut l = 0;
        let mut r = 0;
        for i in 1..n {
            let mut zi = 0;
            if i < r {
                zi = (z[i - l] as usize).min(r - i);
            }
            while i + zi < n && s[zi] == s[i + zi] {
                zi += 1;
            }
            if zi != z[i] as usize {
                return false;
            }
            if i + zi > r {
                l = i;
                r = i + zi;
            }
        }
        true
    }
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();
        let mut s = vec![b' '; n];
        let empty = String::new();
        let mut c = b'a';
        let mut lcp = lcp;
        for i in 0..n {
            if lcp[i][i] as usize + i != n {
                return empty;
            }
            lcp[i][i] = 0;
            if s[i] == b' ' {
                if c <= b'z' {
                    s[i] = c;
                } else {
                    return empty;
                }
                c += 1;
            }
            for j in i + 1..n {
                if lcp[i][j] != lcp[j][i] {
                    return empty;
                }
                if lcp[i][j] > 0 {
                    if s[j] != b' ' && s[j] != s[i] {
                        return empty;
                    }
                    s[j] = s[i];
                }
            }
        }
        for i in 0..n {
            if !Self::check(&s[i..], &(lcp[i][i..])) {
                return empty;
            }
        }
        String::from_utf8(s).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_the_string() {
        assert_eq!(
            Solution::find_the_string(vec_vec![[4, 0, 2, 0], [0, 3, 0, 1], [2, 0, 2, 0], [0, 1, 0, 1]]),
            "abab"
        );
        assert_eq!(
            Solution::find_the_string(vec_vec![[4, 3, 2, 1], [3, 3, 2, 1], [2, 2, 2, 1], [1, 1, 1, 1]]),
            "aaaa"
        );
        assert_eq!(
            Solution::find_the_string(vec_vec![[4, 3, 2, 1], [3, 3, 2, 1], [2, 2, 2, 1], [1, 1, 1, 3]]),
            ""
        );
    }
}
