// https://leetcode.com/problems/maximum-69-number/
// 1323. Maximum 69 Number
pub struct Solution;
impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut div = 1000;
        let mut n = num;
        while div > 0 {
            let d = n / div;
            if d == 6 {
                return num + div * 3;
            }
            n = n % div;
            div /= 10;
        }
        num
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum69_number() {
        assert_eq!(Solution::maximum69_number(9669), 9969);
        assert_eq!(Solution::maximum69_number(9996), 9999);
        assert_eq!(Solution::maximum69_number(9999), 9999);
    }
}
