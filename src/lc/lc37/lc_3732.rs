// https://leetcode.com/problems/maximum-product-of-three-elements-after-one-replacement/
// 3732. Maximum Product of Three Elements After One Replacement
pub struct Solution;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i64 {
        let mut m1 = 0;
        let mut m2 = 0;
        for n in nums {
            let n = n.abs();
            if n > m2 {
                if n > m1 {
                    m2 = m1;
                    m1 = n;
                } else {
                    m2 = n;
                }
            }
        }
        m1 as i64 * m2 as i64 * 100000
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_product() {
        assert_eq!(Solution::max_product(vec![-5, 7, 0]), 3500000);
        assert_eq!(Solution::max_product(vec![-4, -2, -1, -3]), 1200000);
        assert_eq!(Solution::max_product(vec![0, 10, 0]), 0);
    }
}
