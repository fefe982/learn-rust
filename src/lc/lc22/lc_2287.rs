// https://leetcode.com/problems/rearrange-characters-to-make-target-string/
// 2287. Rearrange Characters to Make Target String
pub struct Solution;
impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut count = [0; 26];
        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }
        let mut count_target = [0; 26];
        for c in target.chars() {
            count_target[(c as u8 - b'a') as usize] += 1;
        }
        let mut ans = i32::MAX;
        for i in 0..26 {
            if count_target[i] > 0 {
                ans = ans.min(count[i] / count_target[i]);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rearrange_characters() {
        assert_eq!(
            Solution::rearrange_characters("ilovecodingonleetcode".to_string(), "code".to_string()),
            2
        );
        assert_eq!(
            Solution::rearrange_characters("abcba".to_string(), "abc".to_string()),
            1
        );
        assert_eq!(
            Solution::rearrange_characters("abbaccaddaeea".to_string(), "aaaaa".to_string()),
            1
        );
    }
}
