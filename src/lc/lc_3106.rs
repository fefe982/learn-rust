// https://leetcode.com/problems/lexicographically-smallest-string-after-operations-with-constraint/
// 3106. Lexicographically Smallest String After Operations With Constraint
pub struct Solution;
impl Solution {
    pub fn get_smallest_string(s: String, k: i32) -> String {
        if k == 0 {
            return s;
        }
        let mut k = k;
        let mut t = "".to_string();
        let a = b'a' as i32;
        for &c in s.as_bytes() {
            let ic = c as i32;
            if ic <= a + 12 && ic - a <= k {
                t.push('a');
                k -= ic - a;
            } else if a + 26 - ic <= k {
                t.push('a');
                k -= a + 26 - ic;
            } else {
                t.push((ic - k) as u8 as char);
                k = 0;
            }
        }
        t
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_smallest_string() {
        assert_eq!(Solution::get_smallest_string("vczp".to_string(), 48), "aaaa");
        assert_eq!(Solution::get_smallest_string("zbbz".to_string(), 3), "aaaz");
        assert_eq!(Solution::get_smallest_string("xaxcd".to_string(), 4), "aawcd");
        assert_eq!(Solution::get_smallest_string("lol".to_string(), 0), "lol");
    }
}
