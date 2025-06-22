// https://leetcode.com/problems/maximum-product-of-three-numbers/
// 628. Maximum Product of Three Numbers
pub struct Solution;
impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut max = [i32::MIN; 3];
        let mut min = [i32::MAX; 3];
        for n in nums {
            if n > max[0] {
                max[2] = max[1];
                max[1] = max[0];
                max[0] = n;
            } else if n > max[1] {
                max[2] = max[1];
                max[1] = n;
            } else if n > max[2] {
                max[2] = n;
            }
            if n < min[0] {
                min[2] = min[1];
                min[1] = min[0];
                min[0] = n;
            } else if n < min[1] {
                min[2] = min[1];
                min[1] = n;
            } else if n < min[2] {
                min[2] = n;
            }
        }
        (max[0] * max[1] * max[2]).max(max[0] * min[0] * min[1])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_product() {
        assert_eq!(Solution::maximum_product(vec![-1, -2, -3, -4]), -6);
        assert_eq!(Solution::maximum_product(vec![1, 2, 3]), 6);
        assert_eq!(Solution::maximum_product(vec![1, 2, 3, 4]), 24);
        assert_eq!(Solution::maximum_product(vec![-1, -2, -3]), -6);
    }
}
