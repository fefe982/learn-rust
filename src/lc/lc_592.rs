// https://leetcode.com/problems/fraction-addition-and-subtraction/
// 592. Fraction Addition and Subtraction
pub struct Solution;
impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        loop {
            if a == 0 {
                return b;
            }
            b %= a;
            if b == 0 {
                return a;
            }
            a %= b;
        }
    }
    pub fn fraction_addition(expression: String) -> String {
        let mut a = 0;
        let mut b = 1;
        let mut na = 0;
        let mut nb = 0;
        let mut sign = 1;
        let mut ca = true;
        for c in expression.chars().chain(['+']) {
            match c {
                '-' | '+' => {
                    if nb != 0 {
                        let sa: i32 = a * nb + b * na;
                        let sb = b * nb;
                        let g = Self::gcd(sa.abs(), sb);
                        a = sa / g;
                        b = sb / g;
                    }
                    if c == '-' {
                        sign = -1;
                    } else {
                        sign = 1;
                    }
                    na = 0;
                    nb = 0;
                    ca = true;
                }
                '/' => {
                    ca = false;
                    na *= sign;
                }
                '0'..='9' => {
                    if ca {
                        na = na * 10 + (c as u8 - b'0') as i32;
                    } else {
                        nb = nb * 10 + (c as u8 - b'0') as i32;
                    }
                }
                _ => unreachable!(),
            }
        }
        format!("{a}/{b}")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fraction_addition() {
        assert_eq!(Solution::fraction_addition(String::from("-1/2+1/2")), "0/1");
        assert_eq!(Solution::fraction_addition(String::from("-1/2+1/2+1/3")), "1/3");
        assert_eq!(Solution::fraction_addition(String::from("1/3-1/2")), "-1/6");
    }
}
