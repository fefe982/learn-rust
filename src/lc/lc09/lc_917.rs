// https://leetcode.com/problems/reverse-only-letters/
// 917. Reverse Only Letters
pub struct Solution;
impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = chars.len() - 1;

        while left < right {
            while left < right && !chars[left].is_ascii_alphabetic() {
                left += 1;
            }
            while left < right && !chars[right].is_ascii_alphabetic() {
                right -= 1;
            }
            if left < right {
                chars.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        chars.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reverse_only_letters() {
        assert_eq!(Solution::reverse_only_letters("ab-cd".to_string()), "dc-ba".to_string());
        assert_eq!(
            Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string()),
            "j-Ih-gfE-dCba".to_string()
        );
    }
}
