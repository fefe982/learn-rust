// https://leetcode.com/problems/maximum-number-of-operations-to-move-ones-to-the-end/
// 3228. Maximum Number of Operations to Move Ones to the End
pub struct Solution;
impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut c1 = 0;
        let mut l = '0';
        let mut res = 0;
        for c in s.chars() {
            if c == '1' {
                c1 += 1;
            } else if c != l {
                res += c1;
            }
            l = c;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_operations() {
        assert_eq!(Solution::max_operations("1001101".to_string()), 4);
        assert_eq!(Solution::max_operations("00111".to_string()), 0);
    }
}
