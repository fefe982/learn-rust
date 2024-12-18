// https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/
// 1475. Final Prices With a Special Discount in a Shop
pub struct Solution;
impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut stk = vec![];
        let mut prices = prices;
        for i in 0..prices.len() {
            while let Some(&(ip, p)) = stk.last() {
                if p >= prices[i] {
                    prices[ip] -= prices[i];
                    stk.pop();
                } else {
                    break;
                }
            }
            stk.push((i, prices[i]));
        }
        prices
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn final_prices() {
        assert_eq!(Solution::final_prices(vec![8, 4, 6, 2, 3]), vec![4, 2, 4, 2, 3]);
        assert_eq!(Solution::final_prices(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
        assert_eq!(Solution::final_prices(vec![10, 1, 1, 6]), vec![9, 0, 1, 6]);
    }
}
