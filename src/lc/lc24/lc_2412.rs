// https://leetcode.com/problems/minimum-money-required-before-transactions/
// 2412. Minimum Money Required Before Transactions
pub struct Solution;
impl Solution {
    pub fn minimum_money(transactions: Vec<Vec<i32>>) -> i64 {
        let mut dec = 0;
        let mut min = 0;
        for trans in transactions {
            if trans[0] > trans[1] {
                dec += (trans[0] - trans[1]) as i64;
                min = min.max(trans[1]);
            } else {
                min = min.max(trans[0]);
            }
        }
        dec + min as i64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn minimum_money() {
        assert_eq!(Solution::minimum_money(vec_vec![[2, 1], [5, 0], [4, 2]]), 10);
        assert_eq!(Solution::minimum_money(vec_vec![[3, 0], [0, 3]]), 3);
    }
}
