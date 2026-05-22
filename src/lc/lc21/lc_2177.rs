// https://leetcode.com/problems/find-three-consecutive-integers-that-sum-to-a-given-number/
// 2177. Find Three Consecutive Integers That Sum to a Given Number
pub struct Solution;
impl Solution {
    pub fn sum_of_three(num: i64) -> Vec<i64> {
        if num % 3 != 0 {
            return vec![];
        }
        let mid = num / 3;
        vec![mid - 1, mid, mid + 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_of_three() {
        assert_eq!(Solution::sum_of_three(33), vec![10, 11, 12]);
        assert_eq!(Solution::sum_of_three(4), vec![]);
    }
}
