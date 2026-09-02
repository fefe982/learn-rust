// https://leetcode.cn/problems/construct-uniform-parity-array-ii/
// 3876. Construct Uniform Parity Array II
pub struct Solution;
impl Solution {
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        let mut has_odd = false;
        let mut min = i32::MAX;
        for n in nums1 {
            if n % 2 == 1 {
                has_odd = true;
            }
            min = min.min(n);
        }
        !has_odd || min % 2 == 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn uniform_array() {
        assert_eq!(Solution::uniform_array(vec![1, 4, 7]), true);
        assert_eq!(Solution::uniform_array(vec![2, 3]), false);
    }
}
