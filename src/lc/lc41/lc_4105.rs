// https://leetcode.com/problems/repeated-string-match-ii/
// 4105. Repeated String Match II
pub struct Solution;
impl Solution {
    pub fn repeated_string_match(a: String, b: String, x: i32, y: i32) -> bool {
        let a = a.as_bytes();
        let b = b.as_bytes();
        let la = a.len();
        let lb = b.len();
        let lxa = la * x as usize;
        let lyb = lb * y as usize;
        if lyb > lxa {
            return false;
        }
        let max_s = la.min(lxa - lyb + 1);
        fn gcd(a: usize, b: usize) -> usize {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        fn lcm(a: usize, b: usize) -> usize {
            a / gcd(a, b) * b
        }
        let pre = lyb.min(lcm(la, lb));
        's: for s in 0..max_s {
            for i in 0..pre {
                if a[(s + i) % la] != b[i % lb] {
                    continue 's;
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
    #[test]
    fn repeated_string_match() {
        assert_eq!(
            Solution::repeated_string_match("ab".to_string(), "abab".to_string(), 2, 1),
            true
        );
        assert_eq!(
            Solution::repeated_string_match("abc".to_string(), "bc".to_string(), 2, 3),
            false
        );
        assert_eq!(
            Solution::repeated_string_match("abcd".to_string(), "cdab".to_string(), 2, 1),
            true
        );
    }
}
