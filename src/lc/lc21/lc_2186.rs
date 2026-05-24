// https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram-ii/
// 2186. Minimum Number of Steps to Make Two Strings Anagram II
pub struct Solution;
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut count = [0i32; 26];
        for b in s.bytes() {
            count[(b - b'a') as usize] += 1;
        }
        for b in t.bytes() {
            count[(b - b'a') as usize] -= 1;
        }
        count.into_iter().map(|x| x.abs()).sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_steps() {
        assert_eq!(Solution::min_steps("leetcode".to_string(), "coats".to_string()), 7);
        assert_eq!(Solution::min_steps("night".to_string(), "thing".to_string()), 0);
    }
}
