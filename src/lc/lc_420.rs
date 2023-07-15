// https://leetcode.com/problems/strong-password-checker/
// 420. Strong Password Checker
pub struct Solution;
impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let password = password.as_bytes();
        let l = password.len();
        let mut low = false;
        let mut up = false;
        let mut digit = false;
        let mut last = 0;
        let mut lc = 1;
        let mut con = 0;
        let mut lc1 = 0;
        let mut lc2 = 0;
        for &b in password {
            if b == last {
                lc += 1;
            } else {
                con += lc / 3;
                if lc >= 3 {
                    if lc % 3 == 0 {
                        lc1 += 1;
                    } else if lc % 3 == 1 {
                        lc2 += 1;
                    }
                }
                last = b;
                lc = 1;
            }
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
                b'.' | b'!' => (),
                _ => unreachable!(),
            }
        }
        con += lc / 3;
        if lc >= 3 {
            if lc % 3 == 0 {
                lc1 += 1;
            } else if lc % 3 == 1 {
                lc2 += 1;
            }
        }
        let mut cat = 0;
        if !low {
            cat += 1;
        }
        if !up {
            cat += 1;
        }
        if !digit {
            cat += 1;
        }
        if l > 20 {
            let mut diff = l as i32 - 20;
            if diff <= lc1 {
                con -= diff;
            } else {
                con -= lc1;
                diff -= lc1;
                if diff / 2 <= lc2 {
                    con -= diff / 2;
                } else {
                    con -= lc2;
                    diff -= lc2 * 2;
                    con -= diff / 3;
                }
            }
            con.max(cat) + (l as i32 - 20)
        } else if l < 6 {
            con.max(cat).max(6 - l as i32)
        } else {
            con.max(cat)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strong_password_checker() {
        assert_eq!(
            Solution::strong_password_checker(String::from("bbaaaaaaaaaaaaaaacccccc")),
            8
        );
        assert_eq!(Solution::strong_password_checker(String::from("aaAA11")), 0);
        assert_eq!(
            Solution::strong_password_checker(String::from("ABABABABABABABABABAB1")),
            2
        );
        assert_eq!(Solution::strong_password_checker(String::from("a")), 5);
        assert_eq!(Solution::strong_password_checker(String::from("aA1")), 3);
        assert_eq!(
            Solution::strong_password_checker(String::from("1337C0d3")),
            0
        );
    }
}
