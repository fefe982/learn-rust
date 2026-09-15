// https://leetcode.com/problems/maximize-amount-after-two-days-of-conversions/
// 3387. Maximize Amount After Two Days of Conversions
pub struct Solution;
impl Solution {
    pub fn max_amount(
        initial_currency: String,
        pairs1: Vec<Vec<String>>,
        rates1: Vec<f64>,
        pairs2: Vec<Vec<String>>,
        rates2: Vec<f64>,
    ) -> f64 {
        let mut best = std::collections::HashMap::new();
        best.insert(initial_currency.clone(), 1.0f64);
        let mut fill = |pairs: &Vec<Vec<String>>, rates: &Vec<f64>| {
            for _ in 0..pairs.len() {
                for i in 0..pairs.len() {
                    let a = &pairs[i][0];
                    let b = &pairs[i][1];
                    best.insert(
                        a.clone(),
                        (*best.get(a).unwrap_or(&0.0)).max(*best.get(b).unwrap_or(&0.0) / rates[i]),
                    );
                    best.insert(
                        b.clone(),
                        (*best.get(b).unwrap_or(&0.0)).max(*best.get(a).unwrap_or(&0.0) * rates[i]),
                    );
                }
            }
        };
        fill(&pairs1, &rates1);
        fill(&pairs2, &rates2);
        *best.get(&initial_currency).unwrap_or(&0.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn max_amount() {
        assert_approx_eq!(
            Solution::max_amount(
                "EUR".to_string(),
                vec_vec_str![["EUR", "USD"], ["USD", "JPY"]],
                vec![2.0, 3.0],
                vec_vec_str![["JPY", "USD"], ["USD", "CHF"], ["CHF", "EUR"]],
                vec![4.0, 5.0, 6.0]
            ),
            720.0
        );
        assert_approx_eq!(
            Solution::max_amount(
                "NGN".to_string(),
                vec_vec_str![["NGN", "EUR"]],
                vec![9.0],
                vec_vec_str![["NGN", "EUR"]],
                vec![6.0]
            ),
            1.5
        );
        assert_approx_eq!(
            Solution::max_amount(
                "USD".to_string(),
                vec_vec_str![["USD", "EUR"]],
                vec![1.0],
                vec_vec_str![["EUR", "JPY"]],
                vec![10.0]
            ),
            1.0
        );
    }
}
