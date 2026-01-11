// https://leetcode.com/problems/thousand-separator/
// 1556. Thousand Separator
pub struct Solution;
impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let mut s = n.to_string();
        let mut i = s.len();
        while i > 3 {
            i -= 3;
            s.insert(i, '.');
        }
        s
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn thousand_separator() {
        assert_eq!(Solution::thousand_separator(987), String::from("987"));
        assert_eq!(Solution::thousand_separator(1234), String::from("1.234"));
    }
}
