// https://leetcode.com/problems/reverse-degree-of-a-string/
// 3498. Reverse Degree of a String
pub struct Solution;
impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let mut ans = 0;
        for (c, i) in s.chars().zip(1..) {
            let ic = c as i32 - 'a' as i32;
            ans += i * (26 - ic);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reverse_degree() {
        assert_eq!(Solution::reverse_degree("abc".to_string()), 148);
        assert_eq!(Solution::reverse_degree("zaza".to_string()), 160);
    }
}
