// https://leetcode.com/problems/construct-uniform-parity-array-i/
// 3875. Construct Uniform Parity Array I
pub struct Solution;
impl Solution {
    pub fn uniform_array(_nums1: Vec<i32>) -> bool {
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn uniform_array() {
        assert_eq!(Solution::uniform_array(vec![2, 3]), true);
        assert_eq!(Solution::uniform_array(vec![4, 6]), true);
    }
}
