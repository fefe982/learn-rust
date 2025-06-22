// https://leetcode.com/problems/maximum-product-subarray/
// 152. Maximum Product Subarray
pub struct Solution;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        let mut nmax = i32::MIN;
        let mut full = 1;
        let mut cnt = 0;
        for n in nums.into_iter() {
            if n == 0 {
                max = max.max(0);
                if cnt > 1 && full < 0 {
                    max = max.max(full / nmax);
                }
                full = 1;
                cnt = 0;
                nmax = i32::MIN;
            } else {
                cnt += 1;
                full *= n;
                max = max.max(full);
                if full < 0 {
                    nmax = nmax.max(full);
                }
            }
        }
        if cnt > 1 && full < 0 {
            max = max.max(full / nmax);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_product() {
        assert_eq!(Solution::max_product(vec![3, -1, 4]), 4);
        assert_eq!(Solution::max_product(vec![-3, 0, 1, -2]), 1);
        assert_eq!(Solution::max_product(vec![-2]), -2);
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    }
}
