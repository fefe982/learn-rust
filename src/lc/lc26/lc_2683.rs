// https://leetcode.com/problems/neighboring-bitwise-xor/
// 2683. Neighboring Bitwise XOR
pub struct Solution;
impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        derived.iter().sum::<i32>() % 2 == 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_does_valid_array_exist() {
        assert_eq!(Solution::does_valid_array_exist(vec![1, 1, 0]), true);
        assert_eq!(Solution::does_valid_array_exist(vec![1, 1]), true);
        assert_eq!(Solution::does_valid_array_exist(vec![1, 0]), false);
    }
}
