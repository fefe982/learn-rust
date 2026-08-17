// https://leetcode.com/problems/minimum-time-to-revert-word-to-initial-state-i/
// 3029. Minimum Time to Revert Word to Initial State
pub struct Solution;
impl Solution {
    pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
        let word = word.as_bytes();
        let mut i = 1;
        let len = word.len() as i32;
        while i * k < word.len() as i32 {
            if word[..(len - i * k) as usize] == word[(i * k) as usize..] {
                return i;
            }
            i += 1;
        }
        i
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_time_to_initial_state() {
        assert_eq!(Solution::minimum_time_to_initial_state("abccba".to_string(), 3), 2);
        assert_eq!(Solution::minimum_time_to_initial_state("abccba".to_string(), 2), 3);
    }
}
