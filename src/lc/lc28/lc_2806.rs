// https://leetcode.com/problems/account-balance-after-rounded-purchase/
// 2806. Account Balance After Rounded Purchase
pub struct Solution;
impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        100 - (purchase_amount + 5) / 10 * 10
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_account_balance_after_purchase() {
        assert_eq!(Solution::account_balance_after_purchase(9), 90);
        assert_eq!(Solution::account_balance_after_purchase(15), 80);
    }
}
