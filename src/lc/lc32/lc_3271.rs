// https://leetcode.com/problems/hash-divided-string/
// 3271. Hash Divided String
pub struct Solution;
impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        let s = s.as_bytes();
        let mut ans = String::new();
        let k = k as usize;
        for i in (0..s.len()).step_by(k) {
            let mut sum = 0;
            for j in i..i + k {
                sum += (s[j] - b'a') as i32;
            }
            ans.push((b'a' + (sum % 26) as u8) as char);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn string_hash() {
        assert_eq!(Solution::string_hash("abcd".to_string(), 2), "bf".to_string());
        assert_eq!(Solution::string_hash("mxz".to_string(), 3), "i".to_string());
    }
}
