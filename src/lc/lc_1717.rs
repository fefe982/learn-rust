// https://leetcode.com/problems/maximum-score-from-removing-substrings/
// 1717. Maximum Score From Removing Substrings
pub struct Solution;
impl Solution {
    fn process(s: &[u8], a: u8, x: i32, y: i32) -> i32 {
        let mut v = vec![];
        let mut res = 0;
        let mut ca = 0;
        let mut cb = 0;
        for &c in s {
            if c == a {
                ca += 1;
                v.push(c);
            } else {
                if v.last().copied().unwrap_or(c) == a {
                    ca -= 1;
                    v.pop();
                    res += x;
                } else {
                    v.push(c);
                    cb += 1;
                }
            }
        }
        res + ca.min(cb) * y
    }
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let s = s.as_bytes();
        let (a, x, y) = if x >= y { (b'a', x, y) } else { (b'b', y, x) };
        let mut res = 0;
        let mut last = b'.';
        let mut beg = 0;
        for (i, &c) in s.iter().chain([b'.'].iter()).enumerate() {
            if c == b'a' || c == b'b' {
                if last != b'a' && last != b'b' {
                    beg = i;
                }
            } else {
                if last == b'a' || last == b'b' {
                    res += Self::process(&s[beg..i], a, x, y);
                }
            }
            last = c;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_gain() {
        assert_eq!(Solution::maximum_gain("cdbcbbaaabab".to_string(), 4, 5), 19);
        assert_eq!(Solution::maximum_gain("aabbaaxybbaabb".to_string(), 5, 4), 20);
    }
}
