// https://leetcode.com/problems/integer-break/
// 343. Integer Break
pub struct Solution;
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }
        if n == 3 {
            return 2;
        }
        let mut s = vec![0; n as usize + 1];
        s[2] = 2;
        s[3] = 3;
        for i in 4..=n as usize {
            s[i] = i as i32;
            for j in 2..=i - 2 {
                s[i] = s[i].max(s[j] * s[i - j]);
            }
        }
        s[n as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_integer_break() {
        assert_eq!(Solution::integer_break(2), 1);
        assert_eq!(Solution::integer_break(10), 36);
    }
}
