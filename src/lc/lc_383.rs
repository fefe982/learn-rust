// https://leetcode.com/problems/ransom-note/
// 383. Ransom Note
pub struct Solution;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut m = std::collections::HashMap::<char, i32>::new();
        for c in magazine.chars() {
            *m.entry(c).or_default() += 1;
        }
        for c in ransom_note.chars() {
            if let Some(cnt) = m.get_mut(&c) {
                if *cnt <= 0 {
                    return false;
                }
                *cnt -= 1;
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
    fn test_can_construct() {
        assert_eq!(Solution::can_construct(String::from("a"), String::from("b")), false);
        assert_eq!(Solution::can_construct(String::from("aa"), String::from("ab")), false);
        assert_eq!(Solution::can_construct(String::from("aa"), String::from("aab")), true);
    }
}
