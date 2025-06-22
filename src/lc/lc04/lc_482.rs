// https://leetcode.com/problems/license-key-formatting/
// 482. License Key Formatting
pub struct Solution;
impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut v = Vec::with_capacity(s.len());
        for c in s.chars() {
            if c != '-' {
                v.push(c.to_ascii_uppercase());
            }
        }
        let len = v.len();
        let k = k as usize;
        let mut res = String::with_capacity(len + len / k);
        let first = len % k;
        for i in 0..first {
            res.push(v[i]);
        }
        let mut i = first;
        while i < len {
            if i != 0 {
                res.push('-');
            }
            for j in i..i + k {
                res.push(v[j]);
            }
            i += k;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn license_key_formatting() {
        assert_eq!(
            Solution::license_key_formatting(String::from("5F3Z-2e-9-w"), 4),
            "5F3Z-2E9W"
        );
        assert_eq!(Solution::license_key_formatting(String::from("2-5g-3-J"), 2), "2-5G-3J");
    }
}
