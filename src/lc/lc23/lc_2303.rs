// https://leetcode.com/problems/calculate-amount-paid-in-taxes/
// 2303. Calculate Amount Paid in Taxes
pub struct Solution;
impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut tax = 0.0;
        let mut last = 0;
        for b in brackets {
            let (up, rate) = (b[0], b[1]);
            if income <= up {
                tax += (income - last) as f64 * rate as f64 / 100.0;
                break;
            } else {
                tax += (up - last) as f64 * rate as f64 / 100.0;
                last = up;
            }
        }
        tax
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn test_calculate_tax() {
        assert_approx_eq!(
            Solution::calculate_tax(vec_vec![[3, 50], [7, 10], [12, 25]], 10),
            2.65000,
            1e-5
        );
        assert_approx_eq!(
            Solution::calculate_tax(vec_vec![[1, 0], [4, 25], [5, 50]], 2),
            0.25000,
            1e-5
        );
        assert_approx_eq!(Solution::calculate_tax(vec_vec![[2, 50]], 0), 0.00000, 1e-5);
    }
}
