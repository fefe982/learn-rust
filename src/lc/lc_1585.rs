// https://leetcode.com/problems/check-if-string-is-transformable-with-substring-sort-operations/
// 1585. Check If String Is Transformable With Substring Sort Operations
pub struct Solution;
impl Solution {
    pub fn is_transformable(s: String, t: String) -> bool {
        let mut places = vec![vec![]; 10];
        for (i, c) in s.bytes().enumerate().rev() {
            places[(c - b'0') as usize].push(i);
        }
        for c in t.bytes() {
            if let Some(p) = places[(c - b'0') as usize].pop() {
                for l in 0..(c - b'0') as usize {
                    if let Some(&lp) = places[l].last() {
                        if p >= lp {
                            return false;
                        }
                    }
                }
            } else {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_transformable() {
        assert_eq!(
            Solution::is_transformable("84532".to_string(), "34852".to_string()),
            true
        );
        assert_eq!(
            Solution::is_transformable("34521".to_string(), "23415".to_string()),
            true
        );
        assert_eq!(
            Solution::is_transformable("12345".to_string(), "12435".to_string()),
            false
        );
    }
}
