// https://leetcode.com/problems/reverse-string/
// 344. Reverse String
pub struct Solution;
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let l = s.len();
        for i in 0..l / 2 {
            let c = s[i];
            s[i] = s[l - i - 1];
            s[l - i - 1] = c;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn reverse_string() {
        let mut v = vec_chr!["h", "e", "l", "l", "o"];
        Solution::reverse_string(&mut v);
        assert_eq!(v, vec_chr!["o", "l", "l", "e", "h"]);
        v = vec_chr!["H", "a", "n", "n", "a", "h"];
        Solution::reverse_string(&mut v);
        assert_eq!(v, vec_chr!["h", "a", "n", "n", "a", "H"]);
    }
}
