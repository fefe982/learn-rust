// https://leetcode.com/problems/richest-customer-wealth/
// 1672. Richest Customer Wealth
pub struct Solution;
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|a| a.iter().sum()).max().unwrap_or(0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximum_wealth() {
        assert_eq!(Solution::maximum_wealth(vec_vec![[1, 2, 3], [3, 2, 1]]), 6);
        assert_eq!(Solution::maximum_wealth(vec_vec![[1, 5], [7, 3], [3, 5]]), 10);
        assert_eq!(Solution::maximum_wealth(vec_vec![[2, 8, 7], [7, 1, 3], [1, 9, 5]]), 17);
    }
}
