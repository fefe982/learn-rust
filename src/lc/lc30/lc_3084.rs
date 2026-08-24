// https://leetcode.com/problems/count-substrings-starting-and-ending-with-given-character/
// 3084. Count Substrings Starting and Ending with Given Character
pub struct Solution;
impl Solution {
    pub fn count_substrings(s: String, c: char) -> i64 {
        let mut cnt = 0;
        for cc in s.chars() {
            if cc == c {
                cnt += 1;
            }
        }
        cnt * (cnt - 1) / 2 + cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_substrings() {
        assert_eq!(Solution::count_substrings("abada".to_string(), 'a'), 6);
        assert_eq!(Solution::count_substrings("zzz".to_string(), 'z'), 6);
    }
}
