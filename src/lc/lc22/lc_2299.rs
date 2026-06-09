// https://leetcode.com/problems/strong-password-checker-ii/
// 2299. Strong Password Checker II
pub struct Solution;
impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        let password = password.as_bytes();
        if password.len() < 8 {
            return false;
        }
        let mut low = false;
        let mut up = false;
        let mut digit = false;
        let mut sym = false;
        let mut last = 0;
        for &b in password {
            if b == last {
                return false;
            }
            last = b;
            match b {
                b'a'..=b'z' => {
                    low = true;
                }
                b'A'..=b'Z' => {
                    up = true;
                }
                b'0'..=b'9' => {
                    digit = true;
                }
                _ => {
                    sym = true;
                }
            }
        }
        low && up && digit && sym
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strong_password_checker_ii() {
        assert_eq!(Solution::strong_password_checker_ii("IloveLe3tcode!".to_string()), true);
        assert_eq!(
            Solution::strong_password_checker_ii("Me+You--IsMyDream".to_string()),
            false
        );
        assert_eq!(Solution::strong_password_checker_ii("1aB!".to_string()), false);
    }
}
