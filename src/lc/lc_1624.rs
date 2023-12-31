// https://leetcode.com/problems/largest-substring-between-two-equal-characters/
// 1624. Largest Substring Between Two Equal Characters
pub struct Solution;
impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut first = vec![-1; 26];
        let mut last = vec![0; 26];
        for (i, c) in s.bytes().enumerate() {
            let n = (c - b'a') as usize;
            if first[n] == -1 {
                first[n] = i as i32;
            } else {
                last[n] = i as i32;
            }
        }
        let mut m = -1;
        for i in 0..26 {
            if last[i] > 0 {
                m = m.max(last[i] - first[i] - 1);
            }
        }
        m
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_length_between_equal_characters() {
        assert_eq!(Solution::max_length_between_equal_characters(String::from("aa")), 0);
        assert_eq!(Solution::max_length_between_equal_characters(String::from("abca")), 2);
        assert_eq!(Solution::max_length_between_equal_characters(String::from("cbzxy")), -1);
    }
}
