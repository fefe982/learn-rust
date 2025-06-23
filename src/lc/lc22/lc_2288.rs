// https://leetcode.com/problems/apply-discount-to-prices/
// 2288. Apply Discount to Prices
pub struct Solution;
impl Solution {
    pub fn discount_prices(sentence: String, discount: i32) -> String {
        sentence
            .split_ascii_whitespace()
            .map(|s| {
                if s.starts_with('$') {
                    s[1..]
                        .parse::<i64>()
                        .map(|x| format!("${:.2}", x as f64 * (100 - discount) as f64 / 100.0))
                        .unwrap_or_else(|_| s.to_string())
                } else {
                    s.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_discount_prices() {
        assert_eq!(
            Solution::discount_prices("there are $1 $2 and 5$ candies in the shop".to_string(), 50),
            "there are $0.50 $1.00 and 5$ candies in the shop"
        );
        assert_eq!(
            Solution::discount_prices("1 2 $3 4 $5 $6 7 8$ $9 $10$".to_string(), 100),
            "1 2 $0.00 4 $0.00 $0.00 7 8$ $0.00 $10$"
        );
    }
}
