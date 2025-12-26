// https://leetcode.com/problems/minimum-number-of-coins-for-fruits/
// 2944. Minimum Number of Coins for Fruit
pub struct Solution;
impl Solution {
    pub fn minimum_coins(prices: Vec<i32>) -> i32 {
        let mut q = std::collections::VecDeque::new();
        for (i, &p) in prices.iter().enumerate() {
            let mut last = 0;
            while let Some(&(j, pj)) = q.front() {
                if j + 1 < i {
                    q.pop_front();
                } else {
                    last = pj;
                    break;
                }
            }
            let ncost = last + p;
            let ni = i * 2 + 1;
            while let Some(&(_, pj)) = q.back() {
                if pj >= ncost {
                    q.pop_back();
                } else {
                    break;
                }
            }
            q.push_back((ni, ncost));
        }
        while let Some(&(j, pj)) = q.front() {
            if j < prices.len() - 1 {
                q.pop_front();
            } else {
                return pj;
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_coins() {
        assert_eq!(Solution::minimum_coins(vec![3, 1, 2]), 4);
        assert_eq!(Solution::minimum_coins(vec![1, 10, 1, 1]), 2);
        assert_eq!(Solution::minimum_coins(vec![26, 18, 6, 12, 49, 7, 45, 45]), 39);
    }
}
