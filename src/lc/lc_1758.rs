// https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/
// 1758. Minimum Changes To Make Alternating Binary String

pub struct Solution;
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let digit = ['0', '1'];
        let mut c0 = 0;
        let mut c1 = 0;
        for (idx, c) in s.chars().enumerate() {
            let i = idx % 2;
            if c == digit[i] {
                c0 += 1;
            } else {
                c1 += 1;
            }
        }
        if c0 < c1 {
            c0
        } else {
            c1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations("0100".to_string()), 1);
        assert_eq!(Solution::min_operations("10".to_string()), 0);
        assert_eq!(Solution::min_operations("1111".to_string()), 2);
    }
}
