// https://leetcode.com/problems/minimum-amount-of-time-to-fill-cups/
// 2335. Minimum Amount of Time to Fill Cups
pub struct Solution;
impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let mut amount = amount;
        amount.sort_unstable();
        if amount[2] >= amount[0] + amount[1] {
            return amount[2];
        }
        (amount[0] + amount[1] + amount[2] + 1) / 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fill_cups() {
        assert_eq!(Solution::fill_cups(vec![1, 4, 2]), 4);
        assert_eq!(Solution::fill_cups(vec![5, 4, 4]), 7);
        assert_eq!(Solution::fill_cups(vec![5, 0, 0]), 5);
    }
}
