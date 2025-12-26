// https://leetcode.cn/problems/minimum-number-of-coins-to-be-added/
// 2952. Minimum Number of Coins to be Added
pub struct Solution;
impl Solution {
    pub fn minimum_added_coins(coins: Vec<i32>, target: i32) -> i32 {
        let mut coins = coins;
        coins.sort_unstable();
        let mut sum = 0;
        let mut add = 0;
        for c in coins {
            if sum >= target {
                break;
            }
            while sum < target && c > sum + 1 {
                add += 1;
                sum += sum + 1;
            }
            sum += c;
        }
        while sum < target {
            add += 1;
            sum += sum + 1;
        }
        add
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_added_coins() {
        assert_eq!(Solution::minimum_added_coins(vec![1, 4, 10], 19), 2);
        assert_eq!(Solution::minimum_added_coins(vec![1, 4, 10, 5, 7, 19], 19), 1);
        assert_eq!(Solution::minimum_added_coins(vec![1, 1, 1], 20), 3);
    }
}
