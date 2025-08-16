// https://leetcode.com/problems/clumsy-factorial/
// 1006. Clumsy Factorial
pub struct Solution;
impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        if n == 1 {
            1
        } else if n == 2 {
            2
        } else if n == 3 {
            6
        } else if n == 4 {
            7
        } else {
            let mut sum = 2 * n - 2;
            let mut n = n - 4;
            sum -= n / 4 * 4;
            n %= 4;
            if n == 0 || n == 1 {
                sum -= 1;
            } else if n == 2 {
                sum -= 2;
            } else if n == 3 {
                sum -= 6;
            }
            sum
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clumsy() {
        assert_eq!(Solution::clumsy(4), 7);
        assert_eq!(Solution::clumsy(10), 12);
    }
}
