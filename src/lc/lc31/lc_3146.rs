// https://leetcode.com/problems/permutation-difference-between-two-strings/
// 3146. Permutation Difference Between Two Strings
pub struct Solution;
impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut cnt = [0; 26];
        s.chars()
            .enumerate()
            .for_each(|(x, c)| cnt[(c as u8 - b'a') as usize] = x as i32);
        t.chars().enumerate().fold(0, |acc, (x, c)| {
            (cnt[(c as u8 - b'a') as usize] - (x as i32)).abs() + acc
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_permutation_difference() {
        assert_eq!(
            Solution::find_permutation_difference("abc".to_string(), "bac".to_string()),
            2
        );
        assert_eq!(
            Solution::find_permutation_difference("abcde".to_string(), "edbac".to_string()),
            12
        );
    }
}
