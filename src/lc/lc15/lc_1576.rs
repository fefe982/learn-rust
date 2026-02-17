// https://leetcode.com/problems/replace-all-s-to-avoid-consecutive-repeating-characters/
// 1576. Replace All ?'s to Avoid Consecutive Repeating Characters
pub struct Solution;
impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        for i in 0..s.len() {
            if s[i] == '?' {
                if (i == 0 || s[i - 1] != 'a') && (i == s.len() - 1 || s[i + 1] != 'a') {
                    s[i] = 'a';
                } else if (i == 0 || s[i - 1] != 'b') && (i == s.len() - 1 || s[i + 1] != 'b') {
                    s[i] = 'b';
                } else {
                    s[i] = 'c';
                }
            }
        }
        s.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn modify_string() {
        fn test(s: &str) {
            let ans = Solution::modify_string(s.to_string());
            let s = s.as_bytes();
            let a = ans.as_bytes();
            assert!(a.len() == s.len());
            for i in 0..a.len() {
                if s[i] != b'?' {
                    assert_eq!(s[i], a[i]);
                } else {
                    assert!(a[i] >= b'a' && a[i] <= b'z');
                    if i > 0 {
                        assert_ne!(a[i], a[i - 1]);
                    }
                    if i < a.len() - 1 {
                        assert_ne!(a[i], a[i + 1]);
                    }
                }
            }
        }
        test("?zs");
        test("ubv?w");
    }
}
