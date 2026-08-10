// https://leetcode.com/problems/make-three-strings-equal/
// 2937. Make Three Equal Strings
pub struct Solution;
impl Solution {
    pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let total = s1.len() + s2.len() + s3.len();
        let mut s = 0;
        for i in 0..s1.len().min(s2.len()).min(s3.len()) {
            if s1[i] != s2[i] || s1[i] != s3[i] {
                break;
            }
            s += 1;
        }
        if s == 0 {
            -1
        } else {
            total as i32 - s * 3
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_minimum_operations() {
        assert_eq!(
            Solution::find_minimum_operations("abc".to_string(), "abb".to_string(), "ab".to_string()),
            2
        );
        assert_eq!(
            Solution::find_minimum_operations("dac".to_string(), "bac".to_string(), "cac".to_string()),
            -1
        );
    }
}
