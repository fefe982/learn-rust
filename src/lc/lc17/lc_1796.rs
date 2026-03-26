// https://leetcode.com/problems/second-largest-digit-in-a-string/
// 1796. Second Largest Digit in a String
pub struct Solution;
impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut first = -1;
        let mut second = -1;

        for b in s.bytes() {
            if !b.is_ascii_digit() {
                continue;
            }

            let d = (b - b'0') as i32;
            if d > first {
                second = first;
                first = d;
            } else if d < first && d > second {
                second = d;
            }
        }

        second
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn second_highest() {
        assert_eq!(Solution::second_highest("dfa12321afd".to_string()), 2);
        assert_eq!(Solution::second_highest("abc1111".to_string()), -1);
    }
}
