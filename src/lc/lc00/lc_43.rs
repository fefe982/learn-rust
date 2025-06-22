// https://leetcode.cn/problems/multiply-strings/
// 43. Multiply Strings
pub struct Solution;
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1 = num1
            .as_bytes()
            .iter()
            .map(|b| (b - b'0') as i32)
            .rev()
            .collect::<Vec<_>>();
        let num2 = num2
            .as_bytes()
            .iter()
            .map(|b| (b - b'0') as i32)
            .rev()
            .collect::<Vec<_>>();
        let mut res = vec![0; num1.len() + num2.len()];
        for i in 0..num1.len() {
            for j in 0..num2.len() {
                res[i + j] += num1[i] * num2[j];
            }
        }
        for i in 0..res.len() - 1 {
            res[i + 1] += res[i] / 10;
            res[i] %= 10;
        }
        for i in (0..res.len()).rev() {
            if res[i] != 0 {
                return res[..=i]
                    .iter()
                    .rev()
                    .map(|&n| (n as u8 + b'0') as char)
                    .collect::<String>();
            }
        }
        "0".to_string()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_multiply() {
        assert_eq!(Solution::multiply("2".to_string(), "3".to_string()), "6".to_string());
        assert_eq!(
            Solution::multiply("123".to_string(), "456".to_string()),
            "56088".to_string()
        );
    }
}
