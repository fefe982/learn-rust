// https://leetcode.com/problems/reformat-the-string/
// 1417. Reformat The String
pub struct Solution;
impl Solution {
    pub fn reformat(s: String) -> String {
        let mut vc = vec![];
        let mut vd = vec![];
        for c in s.chars() {
            if c.is_ascii_alphabetic() {
                vc.push(c);
            } else {
                vd.push(c);
            }
        }
        if (vc.len() as i32 - vd.len() as i32).abs() > 1 {
            return String::new();
        }
        if (vd.len() as i32 - vc.len() as i32) > 1 {
            return String::new();
        }
        let mut i = 0;
        let mut j = 0;
        let mut ans = String::with_capacity(s.len());
        if vc.len() > vd.len() {
            ans.push(vc[0]);
            i += 1;
        }
        while i < vc.len() && j < vd.len() {
            ans.push(vd[j]);
            ans.push(vc[i]);
            i += 1;
            j += 1;
        }
        if j < vd.len() {
            ans.push(vd[j]);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(s: &str, e: &str) {
        let a = Solution::reformat(s.to_string());
        assert_eq!(a.len(), e.len());
        let mut map = std::collections::HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        let mut last = '.';
        for c in a.chars() {
            let cnt = map.get_mut(&c).unwrap();
            *cnt -= 1;
            assert!(*cnt >= 0);
            assert!(
                last == '.'
                    || (last.is_ascii_alphabetic() && c.is_ascii_digit())
                    || (last.is_ascii_digit() && c.is_ascii_alphabetic())
            );
            last = c;
        }
    }
    #[test]
    fn reformat() {
        check("a0b1c2", "0a1b2c");
        check("leetcode", "");
        check("1229857369", "");
    }
}
