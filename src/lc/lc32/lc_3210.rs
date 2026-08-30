// https://leetcode.com/problems/find-the-encrypted-string/
// 3210. Find the Encrypted String
pub struct Solution;
impl Solution {
    pub fn get_encrypted_string(s: String, k: i32) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let k = k as usize % s.len();
        s.rotate_left(k);
        s.iter().collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_encrypted_string() {
        assert_eq!(
            Solution::get_encrypted_string("dart".to_string(), 3),
            "tdar".to_string()
        );
        assert_eq!(Solution::get_encrypted_string("aaa".to_string(), 1), "aaa".to_string());
    }
}
