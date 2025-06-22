// https://leetcode.com/problems/1-bit-and-2-bit-characters/
// 717. 1-bit and 2-bit Characters
pub struct Solution;
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = 0;
        while i < bits.len() - 1 {
            i += bits[i] as usize + 1;
        }
        i == bits.len() - 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_one_bit_character() {
        assert_eq!(Solution::is_one_bit_character(vec![1, 0, 0]), true);
        assert_eq!(Solution::is_one_bit_character(vec![1, 1, 1, 0]), false);
    }
}
