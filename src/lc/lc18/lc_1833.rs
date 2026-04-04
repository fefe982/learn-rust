// https://leetcode.com/problems/maximum-ice-cream-bars/
// 1833. Maximum Ice Cream Bars
pub struct Solution;
impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut costs = costs;
        costs.sort();
        let mut res = 0;
        let mut coins = coins;
        for cost in costs {
            if coins >= cost {
                res += 1;
                coins -= cost;
            } else {
                break;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_ice_cream() {
        assert_eq!(Solution::max_ice_cream(vec![1, 3, 2, 4, 1], 7), 4);
        assert_eq!(Solution::max_ice_cream(vec![10, 6, 8, 7, 7, 8], 5), 0);
        assert_eq!(Solution::max_ice_cream(vec![1, 6, 3, 1, 2, 5], 20), 6);
    }
}
