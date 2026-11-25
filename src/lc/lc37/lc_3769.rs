// https://leetcode.com/problems/sort-integers-by-binary-reflection/
// 3769. Sort Integers by Binary Reflection
pub struct Solution;
impl Solution {
    pub fn sort_by_reflection(nums: Vec<i32>) -> Vec<i32> {
        fn rev(n: i32) -> i32 {
            let mut r = 0;
            let mut n = n;
            while n > 0 {
                r = r * 2 + n % 2;
                n /= 2;
            }
            r
        }
        let mut nums = nums;
        nums.sort_by_key(|&x| (rev(x), x));
        nums
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sort_by_reflection() {
        assert_eq!(Solution::sort_by_reflection(vec![4, 5, 4]), vec![4, 4, 5]);
        assert_eq!(Solution::sort_by_reflection(vec![3, 6, 5, 8]), vec![8, 3, 6, 5]);
    }
}
