// https://leetcode.com/problems/array-nesting
// 565. Array Nesting
pub struct Solution;
impl Solution {
    fn get_max(nums: &Vec<i32>, l: &mut Vec<i32>, i: usize, s: i32) -> (i32, usize) {
        if l[i] > 0 {
            return (l[i], usize::MAX);
        }
        if l[i] < 0 {
            return (s + l[i], i);
        }
        l[i] = -s;
        let (len, node) = Self::get_max(nums, l, nums[i] as usize, s + 1);
        if node == usize::MAX {
            l[i] = len + 1;
            return (len + 1, usize::MAX);
        } else {
            l[i] = len;
            if node == i {
                return (len, usize::MAX);
            } else {
                return (len, node);
            }
        }
    }
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut l = vec![0; nums.len()];
        let mut max = 0;
        for i in 0..nums.len() {
            max = max.max(Self::get_max(&nums, &mut l, i, 1).0);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn array_nesting() {
        assert_eq!(Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]), 4);
        assert_eq!(Solution::array_nesting(vec![0, 1, 2]), 1);
    }
}
