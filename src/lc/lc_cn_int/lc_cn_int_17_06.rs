// https://leetcode.com/problems/number-of-2s-in-range-lcci/
// 面试题 17.06. Number Of 2s In Range LCCI
pub struct Solution;
impl Solution {
    pub fn number_of2s_in_range(n: i32) -> i32 {
        let mut n = n;
        let mut d = 9;
        let mut t = 1_000_000_000;
        let mut sum = 0;
        while n > 0 {
            let cd = n / t;
            n = n % t;
            sum += cd * d * (t / 10);
            if cd == 2 {
                sum += n + 1;
            }
            if cd > 2 {
                sum += t;
            }
            d -= 1;
            t /= 10;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of2s_in_range() {
        assert_eq!(Solution::number_of2s_in_range(25), 9);
    }
}
