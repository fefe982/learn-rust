// https://leetcode.com/problems/partition-array-into-two-equal-product-subsets/
// 3566. Partition Array Into Two Equal Product Subsets
pub struct Solution;
impl Solution {
    pub fn check_equal_partitions(nums: Vec<i32>, target: i64) -> bool {
        let n = nums.len();
        for i in 0..n {
            if target % nums[i] as i64 != 0 {
                return false;
            }
        }
        let mut i = 1;
        'i: while i < (1 << (n - 1)) {
            let mut prod1 = 1;
            let mut prod2 = 1;
            for j in (0..n).rev() {
                if i & (1 << j) != 0 {
                    prod1 *= nums[j] as i64;
                } else {
                    prod2 *= nums[j] as i64;
                }
                if prod1 > target || target % prod1 != 0 || prod2 > target || target % prod2 != 0 {
                    i += 1 << j;
                    i &= !((1 << j) - 1);
                    continue 'i;
                }
            }
            if prod1 == target && prod2 == target {
                return true;
            } else {
                return false;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_equal_partitions() {
        assert_eq!(Solution::check_equal_partitions(vec![3, 1, 6, 8, 4], 24), true);
        assert_eq!(Solution::check_equal_partitions(vec![2, 5, 3, 7], 15), false);
    }
}
