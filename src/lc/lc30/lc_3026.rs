// https://leetcode.com/problems/maximum-good-subarray-sum/
// 3026. Maximum Good Subarray Sum
pub struct Solution;
impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut sum = 0;
        let mut m = std::collections::HashMap::new();
        let mut max = i64::MIN;
        for n in nums {
            m.entry(n).and_modify(|v| *v = sum.min(*v)).or_insert(sum);
            sum += n as i64;
            if let Some(s) = m.get(&(n + k)) {
                max = max.max(sum - s);
            }
            if let Some(s) = m.get(&(n - k)) {
                max = max.max(sum - s);
            }
        }
        if max == i64::MIN {
            0
        } else {
            max
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_subarray_sum() {
        assert_eq!(Solution::maximum_subarray_sum(vec![1, 5], 2), 0);
        assert_eq!(Solution::maximum_subarray_sum(vec![1, 2, 3, 4, 5, 6], 1), 11);
        assert_eq!(Solution::maximum_subarray_sum(vec![-1, 3, 2, 4, 5], 3), 11);
        assert_eq!(Solution::maximum_subarray_sum(vec![-1, -2, -3, -4], 2), -6);
    }
}
