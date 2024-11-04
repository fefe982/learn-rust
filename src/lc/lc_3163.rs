// https://leetcode.com/problems/string-compression-iii/
// 3163. String Compression III
pub struct Solution;
impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut cnt = 0;
        let mut prev = ' ';
        let mut res = String::new();
        for c in word.chars() {
            if c != prev || cnt == 9 {
                if cnt > 0 {
                    res.push_str(&format!("{}{}", cnt, prev));
                }
                cnt = 1;
                prev = c;
            } else {
                cnt += 1;
            }
        }
        res.push_str(&format!("{}{}", cnt, prev));
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compressed_string() {
        assert_eq!(
            Solution::compressed_string("abcde".to_string()),
            "1a1b1c1d1e".to_string()
        );
        assert_eq!(
            Solution::compressed_string("aaaaaaaaaaaaaabb".to_string()),
            "9a5a2b".to_string()
        );
    }
}
