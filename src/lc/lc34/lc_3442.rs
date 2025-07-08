// https://leetcode.com/problems/maximum-difference-between-even-and-odd-frequency-i/
// 3442. Maximum Difference Between Even and Odd Frequency
pub struct Solution;
impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut cnt = vec![0; 26];
        s.bytes().for_each(|c| cnt[(c - b'a') as usize] += 1);
        let mut max = 0;
        let mut min = i32::MAX;
        for c in cnt {
            if c % 2 == 1 {
                max = max.max(c);
            } else if c != 0 {
                min = min.min(c);
            }
        }
        max - min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_difference() {
        assert_eq!(Solution::max_difference("aaaaabbc".to_string()), 3);
        assert_eq!(Solution::max_difference("abcabcab".to_string()), 1);
    }
}
