// https://leetcode.com/problems/maximum-unique-subarray-sum-after-deletion/
// 3487. Maximum Unique Subarray Sum After Deletion
pub struct Solution;
impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut c = [0; 101];
        let mut sum = 0;
        let mut m = i32::MIN;
        for i in nums {
            if i > 0 {
                let idx = i as usize;
                c[idx] += 1;
                if c[idx] == 1 {
                    sum += i;
                }
            } else {
                m = m.max(i);
            }
        }
        if sum > 0 {
            sum
        } else {
            m
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_sum() {
        assert_eq!(Solution::max_sum(vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(Solution::max_sum(vec![1, 1, 0, 1, 1]), 1);
        assert_eq!(Solution::max_sum(vec![1, 2, -1, -2, 1, 0, -1]), 3);
    }
}
