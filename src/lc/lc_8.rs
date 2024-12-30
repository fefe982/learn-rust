// https://leetcode.com/problems/string-to-integer-atoi/
// 8. String to Integer (atoi)
pub struct Solution;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut sign = 0;
        let mut res: i32 = 0;
        let mut blank = true;
        for &c in s.as_bytes() {
            match c {
                b' ' => {
                    if !blank {
                        break;
                    }
                }
                b'-' | b'+' => {
                    if sign != 0 {
                        break;
                    } else {
                        sign = if c == b'-' { 1 } else { -1 };
                    }
                    blank = false;
                }
                b'0'..=b'9' => {
                    blank = false;
                    if sign == 0 {
                        sign = -1;
                    }
                    res = res
                        .checked_mul(10)
                        .unwrap_or(i32::MIN)
                        .checked_add(-((c - b'0') as i32))
                        .unwrap_or(i32::MIN);
                }
                _ => break,
            }
        }
        res.checked_mul(sign).unwrap_or(i32::MAX)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn my_atoi() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("   -042".to_string()), -42);
        assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337);
        assert_eq!(Solution::my_atoi("0-1".to_string()), 0);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    }
}
