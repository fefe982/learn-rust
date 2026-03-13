// https://leetcode.com/problems/change-minimum-characters-to-satisfy-one-of-three-conditions/
// 1737. Change Minimum Characters to Satisfy One of Three Conditions
pub struct Solution;
impl Solution {
    pub fn min_characters(a: String, b: String) -> i32 {
        let mut cnta = vec![0; 26];
        let mut cntb = vec![0; 26];
        let a = a.as_bytes();
        let b = b.as_bytes();
        let mut op = i32::MAX;
        for i in 0..a.len() {
            cnta[(a[i] - b'a') as usize] += 1;
        }
        for i in 0..b.len() {
            cntb[(b[i] - b'a') as usize] += 1;
        }
        let la = a.len() as i32;
        let lb = b.len() as i32;
        let mut sa = 0;
        let mut sb = 0;
        let mut maxc = 0;
        for i in 0..26 {
            maxc = maxc.max(cnta[i] + cntb[i]);
            if i == 25 {
                break;
            }
            sa += cnta[i];
            sb += cntb[i];
            op = op.min(sa + lb - sb).min(sb + la - sa);
        }
        op.min(la + lb - maxc)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_characters() {
        assert_eq!(
            Solution::min_characters(
                "a".to_string(),
                "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_string()
            ),
            2
        );
        assert_eq!(Solution::min_characters("azzzz".to_string(), "bzzzz".to_string()), 2);
        assert_eq!(Solution::min_characters("aba".to_string(), "caa".to_string()), 2);
        assert_eq!(Solution::min_characters("dabadd".to_string(), "cda".to_string()), 3);
    }
}
