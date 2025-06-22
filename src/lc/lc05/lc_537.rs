// https://leetcode.com/problems/complex-number-multiplication/
// 537. Complex Number Multiplication
pub struct Solution;
impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let ptn = regex::Regex::new(r"^(-?[0-9]+)\+(-?[0-9]+)i$").unwrap();
        let m1 = ptn.captures(&num1).unwrap();
        let r1 = m1.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let i1 = m1.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let m2 = ptn.captures(&num2).unwrap();
        let r2 = m2.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let i2 = m2.get(2).unwrap().as_str().parse::<i32>().unwrap();
        format!("{}+{}i", r1 * r2 - i1 * i2, r1 * i2 + r2 * i1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn complex_number_multiply() {
        assert_eq!(
            Solution::complex_number_multiply("1+1i".to_string(), "1+1i".to_string()),
            "0+2i".to_string()
        );
        assert_eq!(
            Solution::complex_number_multiply("1+-1i".to_string(), "1+-1i".to_string()),
            "0+-2i".to_string()
        );
    }
}
