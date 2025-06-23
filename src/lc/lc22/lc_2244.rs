// https://leetcode.com/problems/minimum-rounds-to-complete-all-tasks/
// 2244. Minimum Rounds to Complete All Tasks
pub struct Solution;
impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut cnt = std::collections::HashMap::<i32, i32>::new();
        for t in tasks {
            *cnt.entry(t).or_default() += 1;
        }
        let mut sum = 0;
        for &v in cnt.values() {
            if v == 1 {
                return -1;
            }
            sum += v / 3;
            if v % 3 != 0 {
                sum += 1;
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_rounds() {
        assert_eq!(Solution::minimum_rounds(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4]), 4);
        assert_eq!(Solution::minimum_rounds(vec![2, 3, 3]), -1);
    }
}
