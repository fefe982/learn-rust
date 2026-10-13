// https://leetcode.com/problems/find-maximum-number-of-non-intersecting-substrings/
// 3557. Find Maximum Number of Non-Intersecting Substrings
pub struct Solution;
impl Solution {
    pub fn max_substrings(word: String) -> i32 {
        let mut last = vec![usize::MAX; 26];
        let mut cnt = 0;
        for (i, c) in word.chars().enumerate() {
            let idx = (c as u8 - b'a') as usize;
            if last[idx] != usize::MAX && i - last[idx] >= 3 {
                cnt += 1;
                last.fill(usize::MAX);
            } else if (&last)[idx] == usize::MAX {
                last[idx] = i;
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_substrings() {
        assert_eq!(Solution::max_substrings("abcdeafdef".to_string()), 2);
        assert_eq!(Solution::max_substrings("bcdaaaab".to_string()), 1);
    }
}
