// https://leetcode.com/problems/coin-change-ii/
// 518. Coin Change II
pub struct Solution;
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        let mut c = vec![vec![0; coins.len()]; amount + 1];
        c[0][0] = 1;
        for i in 0..=amount {
            for j in 0..coins.len() {
                if j > 0 {
                    c[i][j] = c[i][j - 1];
                }
                if i < coins[j] as usize {
                    continue;
                }
                let idx = i - coins[j] as usize;
                c[i][j] += c[idx][j];
            }
        }
        c[amount][coins.len() - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn change() {
        assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
        assert_eq!(Solution::change(3, vec![2]), 0);
        assert_eq!(Solution::change(10, vec![10]), 1);
    }
}
