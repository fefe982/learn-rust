// https://leetcode.com/problems/find-minimum-cost-to-remove-array-elements/
// 3469. Find Minimum Cost to Remove Array Elements
pub struct Solution;
impl Solution {
    pub fn min_cost(nums: Vec<i32>) -> i32 {
        let mut f = nums.clone();
        let n = nums.len();
        if n % 2 == 0 {
            for i in 0..n - 1 {
                f[i] = f[i].max(f[n - 1]);
            }
        }
        for i in (1..n - 1).step_by(2).rev() {
            let b = nums[i];
            let c = nums[i + 1];
            for j in 0..i {
                let a = nums[j];
                f[j] = (f[j] + b.max(c)).min(f[i] + a.max(c)).min(f[i + 1] + a.max(b));
            }
        }
        f[0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_cost() {
        assert_eq!(Solution::min_cost(vec![6, 2, 8, 4]), 12);
        assert_eq!(Solution::min_cost(vec![2, 1, 3, 3]), 5);
    }
}
