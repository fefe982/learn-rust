// https://leetcode.com/problems/buddy-strings/
// 859. Buddy Strings
pub struct Solution;
impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s == goal {
            let mut cnt = vec![0; 26];
            for c in s.bytes() {
                cnt[(c - b'a') as usize] += 1;
                if cnt[(c - b'a') as usize] >= 2 {
                    return true;
                }
            }
            false
        } else if s.len() != goal.len() {
            false
        } else {
            let mut diff: Option<(u8, u8)> = None;
            for (a, b) in s.bytes().zip(goal.bytes()) {
                if a != b {
                    if let Some((sa, sb)) = diff {
                        if sa == b && sb == a {
                            diff = Some((0, 0));
                        } else {
                            return false;
                        }
                    } else {
                        diff = Some((a, b));
                    }
                }
            }
            if let Some((0, 0)) = diff {
                true
            } else {
                false
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn buddy_strings() {
        assert_eq!(
            Solution::buddy_strings(String::from("ab"), String::from("ba")),
            true
        );
        assert_eq!(
            Solution::buddy_strings(String::from("ab"), String::from("ab")),
            false
        );
        assert_eq!(
            Solution::buddy_strings(String::from("aa"), String::from("aa")),
            true
        );
    }
}
