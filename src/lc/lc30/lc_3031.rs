// https://leetcode.com/problems/minimum-time-to-revert-word-to-initial-state-ii/
// 3031. Minimum Time to Revert Word to Initial State II
pub struct Solution;
impl Solution {
    pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
        let word = word.as_bytes();
        let mut next = vec![0; word.len()];
        let mut i = 0;
        for j in 1..word.len() {
            while word[i] != word[j] && i != 0 {
                i = next[i - 1];
            }
            if word[i] == word[j] {
                i += 1;
            }
            next[j] = i;
        }
        i = word.len();
        let k = k as usize;
        while i > 0 {
            if (word.len() - next[i - 1]) % k == 0 {
                return ((word.len() - next[i - 1]) / k) as i32;
            }
            i = next[i - 1];
        }
        return ((word.len() + k - 1) / k) as i32;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_time_to_initial_state() {
        assert_eq!(Solution::minimum_time_to_initial_state("aab".to_string(), 2), 2);
        assert_eq!(Solution::minimum_time_to_initial_state("abacaba".to_string(), 3), 2);
        assert_eq!(Solution::minimum_time_to_initial_state("abacaba".to_string(), 4), 1);
        assert_eq!(Solution::minimum_time_to_initial_state("abcbabcd".to_string(), 2), 4);
    }
}
