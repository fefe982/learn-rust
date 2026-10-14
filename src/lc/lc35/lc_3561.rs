// https://leetcode.com/problems/resulting-string-after-adjacent-removals/
// 3561. Resulting String After Adjacent Removals
pub struct Solution;
impl Solution {
    pub fn resulting_string(s: String) -> String {
        let mut v = vec![];
        for c in s.chars() {
            if let Some(&x) = v.last() {
                let diff = (x as i32 - c as i32 + 26) % 26;
                if diff == 1 || diff == 25 {
                    v.pop();
                } else {
                    v.push(c);
                }
            } else {
                v.push(c);
            }
        }
        v.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resulting_string() {
        assert_eq!(Solution::resulting_string("abc".to_string()), "c".to_string());
        assert_eq!(Solution::resulting_string("adcb".to_string()), "".to_string());
        assert_eq!(Solution::resulting_string("zadb".to_string()), "db".to_string());
    }
}
