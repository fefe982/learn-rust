// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one/
// 1404. Number of Steps to Reduce a Number in Binary Representation to One
pub struct Solution;
impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut first_chunk_1 = 0;
        let mut mid_zero = 0;
        let mut len = 0;
        let mut chunck_zero = 0;
        let mut first_chunk = true;
        for c in s.chars() {
            len += 1;
            if first_chunk {
                if c == '1' {
                    first_chunk_1 += 1;
                } else {
                    first_chunk = false;
                    chunck_zero += 1;
                    mid_zero += 1;
                }
            } else {
                if c == '0' {
                    mid_zero += 1;
                    chunck_zero += 1;
                } else {
                    chunck_zero = 0;
                }
            }
        }
        mid_zero -= chunck_zero;
        if first_chunk_1 == 1 && mid_zero == 0 {
            len - 1
        } else {
            mid_zero + 1 + len
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_steps() {
        assert_eq!(Solution::num_steps("1101".to_string()), 6);
        assert_eq!(Solution::num_steps("10".to_string()), 1);
        assert_eq!(Solution::num_steps("1".to_string()), 0);
    }
}
