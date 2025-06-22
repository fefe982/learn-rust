// https://leetcode.com/problems/coin-change/
// 322. Coin Change
pub struct Solution;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let mut v = vec![amount + 1; amount as usize + 1];
        v[0] = 0;
        for i in 1..=amount as usize {
            for &coin in &coins {
                if coin <= i as i32 {
                    v[i] = v[i].min(v[i - coin as usize] + 1);
                }
            }
        }
        if v[amount as usize] == amount + 1 {
            -1
        } else {
            v[amount as usize]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_coin_change() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }
}
