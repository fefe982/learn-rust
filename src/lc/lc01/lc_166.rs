// https://leetcode.com/problems/fraction-to-recurring-decimal/
// 166. Fraction to Recurring Decimal
pub struct Solution;
impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_string();
        }
        let mut sign = "";
        if numerator < 0 && denominator > 0 || numerator > 0 && denominator < 0 {
            sign = "-";
        }
        let numerator = (numerator as i64).abs();
        let denominator = (denominator as i64).abs();
        let mut result = sign.to_string() + &(numerator / denominator).to_string();
        let mut remain = (numerator % denominator).abs();
        if remain == 0 {
            return result;
        }
        result.push('.');
        let mut map = std::collections::HashMap::new();
        while remain != 0 {
            if let Some(&index) = map.get(&remain) {
                result.insert(index, '(');
                result.push(')');
                break;
            }
            map.insert(remain, result.len());
            remain *= 10;
            result.push_str(&(remain / denominator).to_string());
            remain %= denominator;
        }
        return result;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fraction_to_decimal() {
        assert_eq!(
            Solution::fraction_to_decimal(-1, -2147483648),
            "0.0000000004656612873077392578125"
        );
        assert_eq!(Solution::fraction_to_decimal(1, 2), "0.5");
        assert_eq!(Solution::fraction_to_decimal(2, 1), "2");
        assert_eq!(Solution::fraction_to_decimal(4, 333), "0.(012)");
    }
}
