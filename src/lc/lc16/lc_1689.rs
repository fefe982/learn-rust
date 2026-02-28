// https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/
// 1689. Partitioning Into Minimum Number Of Deci-Binary Numbers
pub struct Solution;
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars().map(|c| c.to_digit(10).unwrap() as i32).max().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_partitions() {
        assert_eq!(Solution::min_partitions("32".to_string()), 3);
        assert_eq!(Solution::min_partitions("82734".to_string()), 8);
        assert_eq!(Solution::min_partitions("27346209830709182346".to_string()), 9);
    }
}
