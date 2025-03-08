// https://leetcode.com/problems/reverse-vowels-of-a-string
// 345. Reverse Vowels of a String
pub struct Solution;
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            while i < j && !"aeiouAEIOU".contains(s[i]) {
                i += 1;
            }
            while i < j && !"aeiouAEIOU".contains(s[j]) {
                j -= 1;
            }
            if i < j {
                s.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        s.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reverse_vowels() {
        assert_eq!(Solution::reverse_vowels("IceCreAm".to_string()), "AceCreIm".to_string());
        assert_eq!(Solution::reverse_vowels("leetcode".to_string()), "leotcede".to_string());
    }
}
