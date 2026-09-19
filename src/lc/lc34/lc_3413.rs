// https://leetcode.com/problems/maximum-coins-from-k-consecutive-bags/
// 3413. Maximum Coins From K Consecutive Bags
pub struct Solution;
impl Solution {
    pub fn maximum_coins(coins: Vec<Vec<i32>>, k: i32) -> i64 {
        let mut coins = coins;
        coins.sort();
        let mut res = 0;
        let mut i = 0;
        let mut j = 0;
        let mut sum = 0;
        while i < coins.len() {
            while j < coins.len() && coins[j][0] < coins[i][0] + k {
                sum += (coins[j][1] - coins[j][0] + 1) as i64 * coins[j][2] as i64;
                j += 1;
            }
            let mut part = 0;
            if coins[j - 1][1] >= coins[i][0] + k {
                part = (coins[j - 1][1] - (coins[i][0] + k) + 1) as i64 * coins[j - 1][2] as i64;
            }
            res = res.max(sum - part);
            if j == coins.len() && part == 0 {
                break;
            }
            sum -= (coins[i][1] - coins[i][0] + 1) as i64 * coins[i][2] as i64;
            i += 1;
        }
        i = 0;
        j = 0;
        sum = 0;
        while j < coins.len() {
            sum += (coins[j][1] - coins[j][0] + 1) as i64 * coins[j][2] as i64;
            while coins[i][1] <= coins[j][1] - k {
                sum -= (coins[i][1] - coins[i][0] + 1) as i64 * coins[i][2] as i64;
                i += 1;
            }
            let mut part = 0;
            if coins[i][0] <= coins[j][1] - k {
                part = (coins[j][1] - k - coins[i][0] + 1) as i64 * coins[i][2] as i64;
            }
            res = res.max(sum - part);
            j += 1;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximum_coins() {
        assert_eq!(
            Solution::maximum_coins(
                vec_vec![
                    [11, 17, 9],
                    [32, 33, 1],
                    [19, 20, 5],
                    [40, 49, 14],
                    [5, 6, 6],
                    [21, 25, 20],
                    [37, 39, 8]
                ],
                24
            ),
            196
        );
        assert_eq!(
            Solution::maximum_coins(vec_vec![[8, 10, 1], [1, 3, 2], [5, 6, 4]], 4),
            10
        );
        assert_eq!(Solution::maximum_coins(vec_vec![[1, 10, 3]], 2), 6);
    }
}
