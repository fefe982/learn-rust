// https://leetcode.com/problems/number-of-changing-keys/
// 3019. Number of Changing Keys
pub struct Solution;
impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let mut last = ' ';
        s.chars().fold(-1, |acc, c| {
            let lc = c.to_ascii_lowercase();
            if lc != last {
                last = lc;
                acc + 1
            } else {
                acc
            }
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_key_changes() {
        assert_eq!(Solution::count_key_changes("aAbBcC".to_string()), 2);
        assert_eq!(Solution::count_key_changes("AaAaAaaA".to_string()), 0);
    }
}
