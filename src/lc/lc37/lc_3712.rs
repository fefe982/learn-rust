// https://leetcode.com/problems/sum-of-elements-with-frequency-divisible-by-k/
// 3712. Sum of Elements with Frequency Divisible by K
pub struct Solution;
impl Solution {
    pub fn sum_divisible_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut m = std::collections::HashMap::new();
        for i in nums {
            *m.entry(i).or_insert(0) += 1;
        }
        let mut sum = 0;
        for (v, c) in m.iter() {
            if c % k == 0 {
                sum += v * c;
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_divisible_by_k() {
        assert_eq!(Solution::sum_divisible_by_k(vec![1, 2, 2, 3, 3, 3, 3, 4], 2), 16);
        assert_eq!(Solution::sum_divisible_by_k(vec![1, 2, 3, 4, 5], 2), 0);
        assert_eq!(Solution::sum_divisible_by_k(vec![4, 4, 4, 1, 2, 3], 3), 12);
    }
}
