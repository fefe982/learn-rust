// https://leetcode.com/problems/maximum-number-of-coins-you-can-get/
// 1561. Maximum Number of Coins You Can Get
pub struct Solution;
impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        let mut piles = piles;
        piles.sort_unstable();
        let mut sum = 0;
        let n = piles.len() / 3;
        for i in 0..n {
            sum += piles[n + i * 2];
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_coins() {
        assert_eq!(Solution::max_coins(vec![2, 4, 1, 2, 7, 8]), 9);
        assert_eq!(Solution::max_coins(vec![2, 4, 5]), 4);
        assert_eq!(Solution::max_coins(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]), 18);
    }
}
