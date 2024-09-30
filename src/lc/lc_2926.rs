// https://leetcode.com/problems/maximum-balanced-subsequence-sum/
// 2926. Maximum Balanced Subsequence Sum
pub struct Solution;
impl Solution {
    pub fn max_balanced_subsequence_sum(nums: Vec<i32>) -> i64 {
        let mut m = std::collections::BTreeMap::<i32, i64>::new();
        let mut max_minus = i32::MIN;
        let mut max_plus = i64::MIN;
        for (i, n) in nums.into_iter().enumerate() {
            if n < 0 {
                max_minus = max_minus.max(n);
            } else {
                let idx = n - i as i32;
                let sum = if let Some((_, &val)) = m.range(..=idx).next_back() {
                    val + n as i64
                } else {
                    n as i64
                };
                max_plus = max_plus.max(sum);
                m.insert(idx, sum);
                while let Some((&key, &val)) = m.range(idx + 1..).next() {
                    if val <= sum {
                        m.remove(&key);
                    } else {
                        break;
                    }
                }
            }
        }
        max_plus.max(max_minus as i64)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_balanced_subsequence_sum() {
        assert_eq!(Solution::max_balanced_subsequence_sum(vec![3, 3, 5, 6]), 14);
        assert_eq!(Solution::max_balanced_subsequence_sum(vec![5, -1, -3, 8]), 13);
        assert_eq!(Solution::max_balanced_subsequence_sum(vec![-2, -1]), -1);
    }
}
