// https://leetcode.com/problems/distribute-candies-among-children-i/
// 2928. Distribute Candies Among Children I
pub struct Solution;
impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let mut total = (n + 2) * (n + 1) / 2;
        for i in 0..n - limit {
            total -= (i + 1) * 3;
        }
        for i in 0..n - limit * 2 - 1 {
            total += (n - i - limit * 2 - 2 + 1) * 3;
        }
        if n >= limit * 3 + 3 {
            let t = n - limit * 3 - 3;
            total -= (t + 2) * (t + 1) / 2;
        }
        total
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distribute_candies() {
        assert_eq!(Solution::distribute_candies(7, 1), 0);
        assert_eq!(Solution::distribute_candies(4, 1), 0);
        assert_eq!(Solution::distribute_candies(5, 2), 3);
        assert_eq!(Solution::distribute_candies(3, 3), 10);
    }
}
