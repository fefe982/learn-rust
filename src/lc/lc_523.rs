// https://leetcode.com/problems/continuous-subarray-sum/
// 523. Continuous Subarray Sum
pub struct Solution;
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut m = std::collections::HashMap::new();
        m.insert(0, 0);
        let mut s = 0;
        for (n, i) in nums.into_iter().zip(1..) {
            s += n;
            let r = s % k;
            if let Some(&j) = m.get(&r) {
                if i - j > 1 {
                    return true;
                }
            } else {
                m.insert(r, i);
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_subarray_sum() {
        assert_eq!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6), true);
        assert_eq!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 13), false);
    }
}
