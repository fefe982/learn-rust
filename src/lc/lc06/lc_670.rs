// https://leetcode.com/problems/maximum-swap/
// 670. Maximum Swap
pub struct Solution;
impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut digits = vec![];
        let mut num = num;
        let mut digit_stack = vec![];
        while num > 0 {
            let digit = num % 10;
            num /= 10;
            digits.push(digit);
            if let Some(&(d, _)) = digit_stack.last() {
                if digit <= d {
                    continue;
                }
            }
            digit_stack.push((digit, digits.len() - 1));
        }
        'l: for i in (1..digits.len()).rev() {
            while let Some(&(d, digit_index)) = digit_stack.last() {
                if digit_index >= i {
                    digit_stack.pop();
                    continue;
                }
                if digits[i] < d {
                    digits.swap(i, digit_index);
                    break 'l;
                }
                continue 'l;
            }
            break;
        }
        digits.into_iter().rev().fold(0, |acc, x| acc * 10 + x)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_swap() {
        assert_eq!(Solution::maximum_swap(98368), 98863);
        assert_eq!(Solution::maximum_swap(2736), 7236);
        assert_eq!(Solution::maximum_swap(9973), 9973);
    }
}
