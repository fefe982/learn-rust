// https://leetcode.com/problems/maximum-number-of-weeks-for-which-you-can-work/
// 1953. Maximum Number of Weeks for Which You Can Work
pub struct Solution;
impl Solution {
    pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
        let (sum, max) = milestones
            .into_iter()
            .fold((0, 0), |(sum, max), f| (sum + f as i64, max.max(f as i64)));
        if sum >= max * 2 {
            sum
        } else {
            (sum - max) * 2 + 1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_weeks() {
        assert_eq!(Solution::number_of_weeks(vec![1, 2, 3]), 6);
        assert_eq!(Solution::number_of_weeks(vec![5, 2, 1]), 7);
    }
}
