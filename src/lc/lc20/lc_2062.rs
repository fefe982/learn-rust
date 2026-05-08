// https://leetcode.com/problems/count-vowel-substrings-of-a-string/
// 2062. Count Vowel Substrings of a String
pub struct Solution;
impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        fn vowel_bit(c: u8) -> i32 {
            match c {
                b'a' => 1,
                b'e' => 2,
                b'i' => 4,
                b'o' => 8,
                b'u' => 16,
                _ => 0,
            }
        }

        let bytes = word.as_bytes();
        let mut ans = 0;
        for i in 0..bytes.len() {
            if vowel_bit(bytes[i]) == 0 {
                continue;
            }
            let mut mask = 0;
            for &c in &bytes[i..] {
                let bit = vowel_bit(c);
                if bit == 0 {
                    break;
                }
                mask |= bit;
                if mask == 31 {
                    ans += 1;
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_vowel_substrings() {
        assert_eq!(Solution::count_vowel_substrings("aeiouu".to_string()), 2);
        assert_eq!(Solution::count_vowel_substrings("unicornarihan".to_string()), 0);
        assert_eq!(Solution::count_vowel_substrings("cuaieuouac".to_string()), 7);
    }
}
