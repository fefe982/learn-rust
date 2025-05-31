// https://leetcode.com/problems/distribute-candies-among-children-ii/
// 2929. Distribute Candies Among Children II
pub struct Solution;
impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        let n = n as i64;
        let limit = limit as i64;
        let mut total = (n + 2) * (n + 1) / 2;
        if n >= limit + 1 {
            let t = n - limit;
            total -= (t + 1) * t / 2 * 3;
        }
        if n >= limit * 2 + 2 {
            let t = n - limit * 2 - 1;
            total += (t + 1) * t / 2 * 3;
        }
        if n >= limit * 3 + 3 {
            let t = n - limit * 3 - 2;
            total -= (t + 1) * t / 2;
        }
        total
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distribute_candies() {
        assert_eq!(Solution::distribute_candies(5, 2), 3);
        assert_eq!(Solution::distribute_candies(3, 3), 10);
    }
}
