// https://leetcode.com/problems/process-string-with-special-operations-ii/
// 3614. Process String With Special Operations II
pub struct Solution;
impl Solution {
    pub fn process_str(s: String, k: i64) -> char {
        let s = s.as_bytes();
        let mut l = 0;
        let mut vl = Vec::with_capacity(s.len());
        for &c in s {
            match c {
                b'*' => {
                    if l > 0 {
                        l -= 1;
                    }
                }
                b'#' => l *= 2,
                b'%' => {}
                _ => l += 1,
            }
            vl.push(l);
        }
        if l <= k {
            return '.';
        }
        let mut k = k;
        for (&c, l) in s.iter().zip(vl.into_iter()).rev() {
            match c {
                b'*' => {}
                b'#' => {
                    if k >= l / 2 {
                        k -= l / 2;
                    }
                }
                b'%' => k = l - k - 1,
                _ => {
                    if k == l - 1 {
                        return c as char;
                    }
                }
            }
        }
        '.'
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn process_str() {
        assert_eq!(Solution::process_str("a#b%*".to_string(), 1), 'a');
        assert_eq!(Solution::process_str("cd%#*#".to_string(), 3), 'd');
    }
}
