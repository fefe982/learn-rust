// https://leetcode.com/problems/find-the-occurrence-of-first-almost-equal-substring/
// 3303. Find the Occurrence of First Almost Equal Substring
pub struct Solution;
impl Solution {
    fn z_function(s: &[u8]) -> Vec<usize> {
        let mut z = vec![0; s.len()];
        let mut l = 0;
        let mut r = 0;
        for i in 1..s.len() {
            if i < r {
                z[i] = z[i - l].min(r - i);
            }
            while i + z[i] < s.len() && s[z[i]] == s[i + z[i]] {
                z[i] += 1;
            }
            if i + z[i] > r {
                l = i;
                r = i + z[i];
            }
        }
        z
    }
    pub fn min_starting_index(s: String, pattern: String) -> i32 {
        let lptn = pattern.len();
        let lstr = s.len();
        let rev = pattern
            .as_bytes()
            .iter()
            .rev()
            .chain(s.as_bytes().iter().rev())
            .cloned()
            .collect::<Vec<_>>();
        let mut fwd = pattern;
        fwd.push_str(&s);
        let fwd = fwd.as_bytes();
        let ifwd = Self::z_function(fwd);
        let irev = Self::z_function(&rev);
        for i in 0..=lstr - lptn {
            if ifwd[lptn + i] + irev[lstr - i] >= lptn - 1 {
                return i as i32;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_starting_index() {
        assert_eq!(
            Solution::min_starting_index("abcdefg".to_string(), "bcdffg".to_string()),
            1
        );
        assert_eq!(
            Solution::min_starting_index("ababbababa".to_string(), "bacaba".to_string()),
            4
        );
        assert_eq!(Solution::min_starting_index("abcd".to_string(), "dba".to_string()), -1);
        assert_eq!(Solution::min_starting_index("dde".to_string(), "d".to_string()), 0);
    }
}
