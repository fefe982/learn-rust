// https://leetcode.com/problems/distribute-candies/
// 575. Distribute Candies
pub struct Solution;
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let n = candy_type.len();
        let t = std::collections::BTreeSet::from_iter(candy_type.into_iter());
        t.len().min(n / 2) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distribute_candies() {
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 3]), 2);
        assert_eq!(Solution::distribute_candies(vec![6, 6, 6, 6]), 1);
    }
}
