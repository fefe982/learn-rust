// https://leetcode.com/problems/maximize-sum-of-squares-of-digits/
// 3723. Maximize Sum of Squares of Array Elements
pub struct Solution;
impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        if (sum + num - 1) / num > 9 {
            return "".to_string();
        }
        let mut ans = "".to_string();
        let mut sum = sum;
        for _ in 0..num {
            if sum > 9 {
                ans.push('9');
                sum -= 9;
            } else if sum > 0 {
                ans.push((sum as u8 + b'0') as char);
                sum = 0;
            } else {
                ans.push('0');
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_sum_of_squares() {
        assert_eq!(Solution::max_sum_of_squares(2, 19), "".to_string());
        assert_eq!(Solution::max_sum_of_squares(2, 3), "30".to_string());
        assert_eq!(Solution::max_sum_of_squares(2, 17), "98".to_string());
        assert_eq!(Solution::max_sum_of_squares(1, 10), "".to_string());
    }
}
