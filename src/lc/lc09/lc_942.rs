// https://leetcode.com/problems/di-string-match/
// 942. DI String Match
pub struct Solution;
impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut res = Vec::with_capacity(s.len() + 1);
        let mut low = 0;
        let mut high = s.len() as i32;
        for c in s.chars() {
            match c {
                'I' => {
                    res.push(low);
                    low += 1;
                }
                'D' => {
                    res.push(high);
                    high -= 1;
                }
                _ => unreachable!(),
            }
        }
        res.push(high);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn di_string_match() {
        assert_eq!(Solution::di_string_match("IDID".to_string()), vec![0, 4, 1, 3, 2]);
        assert_eq!(Solution::di_string_match("III".to_string()), vec![0, 1, 2, 3]);
        assert_eq!(Solution::di_string_match("DDI".to_string()), vec![3, 2, 0, 1]);
    }
}
