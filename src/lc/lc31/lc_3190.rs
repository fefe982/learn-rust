// https://leetcode.com/problems/find-minimum-operations-to-make-all-elements-divisible-by-three/
// 3190. Find Minimum Operations to Make All Elements Divisible by Three
pub struct Solution;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.into_iter().filter(|&x| x % 3 != 0).count() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_operations() {
        assert_eq!(Solution::minimum_operations(vec![1, 2, 3, 4]), 3);
        assert_eq!(Solution::minimum_operations(vec![3, 6, 9]), 0);
    }
}
