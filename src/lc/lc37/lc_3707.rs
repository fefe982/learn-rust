// https://leetcode.com/problems/equal-score-substrings/
// 3707. Equal Score Substrings
pub struct Solution;
impl Solution {
    pub fn score_balance(s: String) -> bool {
        let sum = s.chars().map(|c| c as i32 - 'a' as i32 + 1).sum::<i32>();
        if sum % 2 == 1 {
            return false;
        }
        let mut ss = sum / 2;
        for c in s.chars() {
            let v = c as i32 - 'a' as i32 + 1;
            ss -= v;
            if ss == 0 {
                return true;
            }
            if ss < 0 {
                return false;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn score_balance() {
        assert_eq!(Solution::score_balance("adcb".to_string()), true);
        assert_eq!(Solution::score_balance("bace".to_string()), false);
    }
}
