// https://leetcode.com/problems/check-if-all-characters-have-equal-number-of-occurrences/
// 1941. Check if All Characters Have Equal Number of Occurrences
pub struct Solution;
impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut counts = [0; 26];
        let mut cnt = 0;
        let mut max = 0;
        let mut tot = 0;
        for c in s.chars() {
            tot += 1;
            let idx = (c as u8 - b'a') as usize;
            counts[idx] += 1;
            if counts[idx] > max {
                max = counts[idx];
            }
            if counts[idx] == 1 {
                cnt += 1;
            }
        }
        tot == max * cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn are_occurrences_equal() {
        assert_eq!(Solution::are_occurrences_equal("abacbc".to_string()), true);
        assert_eq!(Solution::are_occurrences_equal("aaabb".to_string()), false);
    }
}
