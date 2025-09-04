// https://leetcode.com/problems/minimum-operations-to-make-the-integer-zero/
// 2749. Minimum Operations to Make the Integer Zero
pub struct Solution;
impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        let num1 = num1 as i64;
        let num2 = num2 as i64;
        for k in 1.. {
            let d = num1 - num2 * k;
            if d <= 0 {
                return -1;
            }
            if d >= k && k >= d.count_ones() as i64 {
                return k as i32;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn make_the_integer_zero() {
        assert_eq!(Solution::make_the_integer_zero(3, -2), 3);
        assert_eq!(Solution::make_the_integer_zero(5, 7), -1);
    }
}
