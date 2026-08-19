// https://leetcode.com/problems/apply-operations-to-make-string-empty/
// 3039. Apply Operations to Make String Empty
pub struct Solution;
impl Solution {
    pub fn last_non_empty_string(s: String) -> String {
        let mut cnt = [0; 26];
        let mut maxc = 0;
        let mut res = "".to_string();
        for c in s.chars() {
            let idx = (c as u8 - b'a') as usize;
            cnt[idx] += 1;
            if cnt[idx] > maxc {
                res = c.to_string();
                maxc = cnt[idx];
            } else if cnt[idx] == maxc {
                res.push(c);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn last_non_empty_string() {
        assert_eq!(
            Solution::last_non_empty_string("aabcbbca".to_string()),
            "ba".to_string()
        );
        assert_eq!(Solution::last_non_empty_string("abcd".to_string()), "abcd".to_string());
    }
}
