// https://leetcode.com/problems/increasing-decreasing-string/
// 1370. Increasing Decreasing String
pub struct Solution;
impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut cnt = vec![0; 26];
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
        let mut ans = String::new();
        while ans.len() < s.len() {
            for i in 0..26 {
                if cnt[i] > 0 {
                    ans.push((i as u8 + b'a') as char);
                    cnt[i] -= 1;
                }
            }
            for i in (0..26).rev() {
                if cnt[i] > 0 {
                    ans.push((i as u8 + b'a') as char);
                    cnt[i] -= 1;
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
    fn sort_string() {
        assert_eq!(
            Solution::sort_string("aaaabbbbcccc".to_string()),
            "abccbaabccba".to_string()
        );
        assert_eq!(Solution::sort_string("rat".to_string()), "art".to_string());
    }
}
