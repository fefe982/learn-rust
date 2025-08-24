// https://leetcode.com/problems/greatest-common-divisor-of-strings/
// 1071. Greatest Common Divisor of Strings
pub struct Solution;
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut l1 = str1.len();
        let mut l2 = str2.len();
        while l1 != 0 && l2 != 0 {
            let l = l1 % l2;
            l1 = l2;
            l2 = l;
        }
        let gcd = l1;
        l1 = str1.len();
        l2 = str2.len();
        let s = str1.chars().take(gcd).collect::<String>();
        if s.repeat(l1 / gcd) != str1 || s.repeat(l2 / gcd) != str2 {
            return "".to_string();
        }
        s
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gcd_of_strings() {
        assert_eq!(
            Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
            "AB".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string()),
            "".to_string()
        );
    }
}
