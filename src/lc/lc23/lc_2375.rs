// https://leetcode.com/problems/construct-smallest-number-from-di-string/
// 2375. Construct Smallest Number From DI String
pub struct Solution;
impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let mut res = "".to_string();
        let mut d = 0;
        let mut nd = 0;
        for c in pattern.chars().chain(std::iter::once('I')) {
            if c == 'I' {
                for i in (0..=nd).rev() {
                    res.push(((d + i + 1) as u8 + b'0') as char);
                }
                d += nd + 1;
                nd = 0;
            } else {
                nd += 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_smallest_number() {
        assert_eq!(
            Solution::smallest_number("IIIDIDDD".to_string()),
            "123549876".to_string()
        );
        assert_eq!(Solution::smallest_number("DDD".to_string()), "4321".to_string());
    }
}
