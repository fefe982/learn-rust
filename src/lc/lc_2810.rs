// https://leetcode.com/problems/faulty-keyboard/
// 2810. Faulty Keyboard
pub struct Solution;
impl Solution {
    pub fn final_string(s: String) -> String {
        let mut q = std::collections::VecDeque::<char>::new();
        let mut dir = 0;
        for c in s.chars() {
            if c == 'i' {
                dir = 1 - dir;
            } else if dir == 0 {
                q.push_back(c);
            } else {
                q.push_front(c);
            }
        }
        if dir == 0 {
            q.iter().collect()
        } else {
            q.iter().rev().collect()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_final_string() {
        assert_eq!(Solution::final_string(String::from("string")), "rtsng");
        assert_eq!(Solution::final_string(String::from("poiinter")), "ponter");
    }
}
