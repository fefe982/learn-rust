// https://leetcode.com/problems/maximum-number-of-consecutive-values-you-can-make/
// 1798. Maximum Number of Consecutive Values You Can Make
pub struct Solution;
impl Solution {
    pub fn get_maximum_consecutive(coins: Vec<i32>) -> i32 {
        let mut coins = coins;
        coins.sort_unstable();
        let mut reachable = 0;
        for coin in coins {
            if coin > reachable + 1 {
                break;
            }
            reachable += coin;
        }
        reachable + 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_maximum_consecutive() {
        assert_eq!(Solution::get_maximum_consecutive(vec![1, 3]), 2);
        assert_eq!(Solution::get_maximum_consecutive(vec![1, 1, 1, 4]), 8);
        assert_eq!(Solution::get_maximum_consecutive(vec![1, 4, 10, 3, 1]), 20);
    }
}
