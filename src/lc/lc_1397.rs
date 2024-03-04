// https://leetcode.com/problems/find-all-good-strings/
// 1397. Find All Good Strings
pub struct Solution;
impl Solution {
    fn lps(s: &[u8]) -> Vec<usize> {
        let mut v = vec![0; s.len()];
        let mut len = 0;
        let mut i = 1;
        while i < s.len() {
            if s[i] == s[len] {
                len += 1;
                v[i] = len;
                i += 1;
            } else if len > 0 {
                len = v[len - 1];
            } else {
                v[i] = 0;
                i += 1;
            }
        }
        v
    }
    fn count(
        n: usize,
        len: usize,
        evil_len: usize,
        use_lowerbound: usize,
        use_upperbound: usize,
        s1: &[u8],
        s2: &[u8],
        evil: &[u8],
        lps: &Vec<usize>,
        mem: &mut Vec<Vec<Vec<Vec<i64>>>>,
    ) -> i64 {
        if evil_len == evil.len() {
            return 0;
        }
        if len == n {
            return 1;
        }
        if mem[len][evil_len][use_lowerbound][use_upperbound] > 0 {
            return mem[len][evil_len][use_lowerbound][use_upperbound];
        }
        let low = if use_lowerbound == 1 { s1[len] } else { b'a' };
        let high = if use_upperbound == 1 { s2[len] } else { b'z' };
        let mut ans = 0;
        for c in low..=high {
            let mut elen = evil_len;
            while elen > 0 && evil[elen] != c {
                elen = lps[elen - 1];
            }
            if evil[elen] == c {
                elen += 1;
            }
            ans += Self::count(
                n,
                len + 1,
                elen,
                if use_lowerbound == 1 && c == s1[len] { 1 } else { 0 },
                if use_upperbound == 1 && c == s2[len] { 1 } else { 0 },
                s1,
                s2,
                evil,
                lps,
                mem,
            );
            ans %= 1_0000_0000_7;
        }
        mem[len][evil_len][use_lowerbound][use_upperbound] = ans;
        ans
    }
    pub fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let evil = evil.as_bytes();
        let lps = Self::lps(&evil);
        Self::count(
            n as usize,
            0,
            0,
            1,
            1,
            s1,
            s2,
            evil,
            &lps,
            &mut vec![vec![vec![vec![0; 2]; 2]; evil.len()]; n as usize],
        ) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_good_strings() {
        assert_eq!(
            Solution::find_good_strings(2, "aa".to_string(), "da".to_string(), "b".to_string()),
            51
        );
        assert_eq!(
            Solution::find_good_strings(8, "leetcode".to_string(), "leetgoes".to_string(), "leet".to_string()),
            0
        );
        assert_eq!(
            Solution::find_good_strings(2, "gx".to_string(), "gz".to_string(), "x".to_string()),
            2
        );
    }
}
