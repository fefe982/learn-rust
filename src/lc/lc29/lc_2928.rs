// https://leetcode.com/problems/distribute-candies-among-children-i/
// 2928. Distribute Candies Among Children I
pub struct Solution;
impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        super::lc_2929::Solution::distribute_candies(n, limit) as i32
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
