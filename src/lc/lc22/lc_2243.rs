// https://leetcode.com/problems/calculate-digit-sum-of-a-string/
// 2243. Calculate Digit Sum of a String
pub struct Solution;
impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let mut sv = s.into_bytes();
        while sv.len() > k as usize {
            let mut new_sv = Vec::new();
            for chunk in sv.chunks(k as usize) {
                let sum = chunk.iter().fold(0, |acc, &b| acc + (b - b'0') as u32);
                new_sv.extend(sum.to_string().into_bytes());
            }
            sv = new_sv;
        }
        String::from_utf8(sv).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn digit_sum() {
        assert_eq!(Solution::digit_sum("11111222223".to_string(), 3), "135".to_string());
        assert_eq!(Solution::digit_sum("00000000".to_string(), 3), "000".to_string());
    }
}
