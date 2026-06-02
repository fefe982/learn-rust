// https://leetcode.com/problems/remove-digit-from-number-to-maximize-result/
// 2259. Remove Digit From Number to Maximize Result
pub struct Solution;
impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let num = number.as_bytes();
        let mut idx = usize::MAX;
        let mut last_idx = 0;
        for i in 0..num.len() {
            if num[i] == digit as u8 {
                if i + 1 < num.len() && num[i + 1] > num[i] {
                    idx = i;
                    break;
                }
                last_idx = i;
            }
        }
        if idx == usize::MAX {
            idx = last_idx;
        }
        let mut ans = Vec::with_capacity(num.len() - 1);
        for i in 0..num.len() {
            if i != idx {
                ans.push(num[i]);
            }
        }
        String::from_utf8(ans).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_digit() {
        assert_eq!(Solution::remove_digit("123".to_string(), '3'), "12");
        assert_eq!(Solution::remove_digit("1231".to_string(), '1'), "231");
        assert_eq!(Solution::remove_digit("551".to_string(), '5'), "51");
    }
}
