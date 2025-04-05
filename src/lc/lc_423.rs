// https://leetcode.com/problems/reconstruct-original-digits-from-english/
// 423. Reconstruct Original Digits from English
pub struct Solution;
impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut cnt = [0; 26];
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
        let mut ans = [0; 10];
        ans[0] = cnt[(b'z' - b'a') as usize];
        ans[2] = cnt[(b'w' - b'a') as usize];
        ans[4] = cnt[(b'u' - b'a') as usize];
        ans[6] = cnt[(b'x' - b'a') as usize];
        ans[8] = cnt[(b'g' - b'a') as usize];
        ans[3] = cnt[(b'h' - b'a') as usize] - ans[8];
        ans[5] = cnt[(b'f' - b'a') as usize] - ans[4];
        ans[7] = cnt[(b's' - b'a') as usize] - ans[6];
        ans[1] = cnt[(b'o' - b'a') as usize] - ans[0] - ans[2] - ans[4];
        ans[9] = (cnt[(b'n' - b'a') as usize] - ans[1] - ans[7]) / 2;
        let mut res = String::new();
        for i in 0..10 {
            res.push_str(&i.to_string().repeat(ans[i]));
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_original_digits() {
        assert_eq!(Solution::original_digits("owoztneoer".to_string()), "012".to_string());
        assert_eq!(Solution::original_digits("fviefuro".to_string()), "45".to_string());
    }
}
