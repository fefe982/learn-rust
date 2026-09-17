// https://leetcode.com/problems/substring-matching-pattern/
// 3407. Substring Matching Pattern
pub struct Solution;
impl Solution {
    pub fn has_match(s: String, p: String) -> bool {
        let Some(star) = p.find('*') else {
            return false;
        };
        let l = &p[..star];
        let r = &p[star + 1..];
        let Some(il) = s.find(l) else {
            return false;
        };
        let Some(ir) = s.rfind(r) else {
            return false;
        };
        il + l.len() <= ir
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn has_match() {
        assert_eq!(Solution::has_match("leetcode".to_string(), "ee*e".to_string()), true);
        assert_eq!(Solution::has_match("car".to_string(), "c*v".to_string()), false);
        assert_eq!(Solution::has_match("luck".to_string(), "u*".to_string()), true);
    }
}
