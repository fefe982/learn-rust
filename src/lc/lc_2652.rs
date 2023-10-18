// https://leetcode.com/problems/sum-multiples/
// 2652. Sum Multiples
pub struct Solution;
impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        let sum = |x, y| {
            let z = x / y;
            z * (z + 1) / 2 * y
        };
        sum(n, 3) + sum(n, 5) + sum(n, 7) - sum(n, 15) - sum(n, 35) - sum(n, 21) + sum(n, 105)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_of_multiples() {
        assert_eq!(Solution::sum_of_multiples(7), 21);
        assert_eq!(Solution::sum_of_multiples(10), 40);
        assert_eq!(Solution::sum_of_multiples(9), 30);
    }
}
