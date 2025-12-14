// https://leetcode.com/problems/number-of-smooth-descent-periods-of-a-stock/
// 2110. Number of Smooth Descent Periods of a Stock
pub struct Solution;
impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut last = 0;
        let mut res = 0;
        let mut l = 0;
        for p in prices {
            if p == last - 1 {
                l += 1;
            } else {
                l = 1;
            }
            res += l as i64;
            last = p;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_descent_periods() {
        assert_eq!(Solution::get_descent_periods(vec![3, 2, 1, 4]), 7);
        assert_eq!(Solution::get_descent_periods(vec![8, 6, 7, 7]), 4);
        assert_eq!(Solution::get_descent_periods(vec![1]), 1);
    }
}
