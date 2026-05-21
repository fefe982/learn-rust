// https://leetcode.com/problems/smallest-value-of-the-rearranged-number/
// 2165. Smallest Value of the Rearranged Number
pub struct Solution;
impl Solution {
    pub fn smallest_number(num: i64) -> i64 {
        if num == 0 {
            return 0;
        }
        let mut digits = num
            .abs()
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        digits.sort_unstable();
        if num > 0 {
            let mut i = 0;
            while digits[i] == 0 {
                i += 1;
            }
            digits.swap(0, i);
        } else {
            digits.reverse();
        }
        digits.into_iter().fold(0, |acc, d| acc * 10 + d as i64) * num.signum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_smallest_number() {
        assert_eq!(Solution::smallest_number(310), 103);
        assert_eq!(Solution::smallest_number(-7605), -7650);
        assert_eq!(Solution::smallest_number(0), 0);
    }
}
