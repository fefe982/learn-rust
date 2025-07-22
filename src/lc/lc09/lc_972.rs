// https://leetcode.com/problems/equal-rational-numbers/
// 972. Equal Rational Numbers
pub struct Solution;
enum State {
    Int,
    Decimal,
    Repeat,
}
impl Solution {
    fn parse_rational(s: String) -> (i64, i64) {
        let mut int_part = 0;
        let mut decimal_denom = 1;
        let mut repeat_part = 0;
        let mut repeat_denom = 1;
        let mut state = State::Int;
        for c in s.chars() {
            match c {
                '0'..='9' => match state {
                    State::Int => int_part = int_part * 10 + (c as i64 - '0' as i64),
                    State::Decimal => {
                        int_part = int_part * 10 + (c as i64 - '0' as i64);
                        decimal_denom = decimal_denom * 10;
                    }
                    State::Repeat => {
                        repeat_part = repeat_part * 10 + (c as i64 - '0' as i64);
                        repeat_denom = repeat_denom * 10;
                    }
                },
                '.' => state = State::Decimal,
                '(' => state = State::Repeat,
                _ => {}
            }
        }
        if repeat_part > 0 {
            int_part = int_part * (repeat_denom - 1) + repeat_part;
            decimal_denom = decimal_denom * (repeat_denom - 1);
        }
        let mut a = int_part;
        let mut b = decimal_denom;
        let gcd = loop {
            if b == 0 {
                break a;
            }
            a = a % b;
            if a == 0 {
                break b;
            }
            b = b % a;
        };
        (int_part / gcd, decimal_denom / gcd)
    }
    pub fn is_rational_equal(s: String, t: String) -> bool {
        Solution::parse_rational(s) == Solution::parse_rational(t)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_rational_equal() {
        assert_eq!(
            Solution::is_rational_equal("0.(52)".to_string(), "0.5(25)".to_string()),
            true
        );
        assert_eq!(
            Solution::is_rational_equal("0.1666(6)".to_string(), "0.166(66)".to_string()),
            true
        );
        assert_eq!(
            Solution::is_rational_equal("0.9(9)".to_string(), "1.".to_string()),
            true
        );
    }
}
