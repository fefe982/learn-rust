// https://leetcode.com/problems/largest-3-same-digit-number-in-string/
// 2264. Largest 3-Same-Digit Number in String
pub struct Solution;
impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut cnt = 0;
        let mut last = ' ';
        let mut max = '\0';
        for c in num.chars() {
            if c == last {
                cnt += 1;
            } else {
                last = c;
                cnt = 1;
            }
            if cnt == 3 && c > max {
                max = c;
            }
        }
        if max == '\0' {
            "".to_string()
        } else {
            std::iter::repeat(max).take(3).collect::<String>()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_largest_good_integer() {
        assert_eq!(Solution::largest_good_integer("6777133339".to_string()), "777");
        assert_eq!(Solution::largest_good_integer("2300019".to_string()), "000");
        assert_eq!(Solution::largest_good_integer("42352338".to_string()), "");
    }
}
