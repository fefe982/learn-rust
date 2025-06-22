// https://leetcode.com/problems/rotate-string/
// 796. Rotate String
pub struct Solution;
impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let s = s.as_bytes();
        let goal = goal.as_bytes();
        if s.len() != goal.len() {
            return false;
        }
        let mut sh = 0;
        let mut gh = 0;
        let md = 1_0000_0000_7i64;
        let mut mul = 1;
        for i in 0..s.len() {
            sh = (sh + mul * (s[i] - b'a') as i64) % md;
            gh = (gh + mul * (goal[i] - b'a') as i64) % md;
            mul = (mul * 26) % md;
        }
        for i in (0..s.len()).rev() {
            if sh == gh {
                if s[..i + 1] == goal[goal.len() - i - 1..] && s[i + 1..] == goal[..goal.len() - i - 1] {
                    return true;
                }
            }
            let mv = (s[i] - b'a') as i64;
            sh = ((sh * 26 - mul * mv) % md + mv + md) % md;
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rotate_string() {
        assert_eq!(Solution::rotate_string("abcde".to_string(), "cdeab".to_string()), true);
        assert_eq!(Solution::rotate_string("abcde".to_string(), "abced".to_string()), false);
    }
}
