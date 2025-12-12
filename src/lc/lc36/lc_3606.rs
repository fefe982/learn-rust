// https://leetcode.com/problems/coupon-code-validator/
// 3606. Coupon Code Validator
pub struct Solution;
impl Solution {
    pub fn validate_coupons(code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
        let mut res = code
            .into_iter()
            .zip(business_line.into_iter().zip(is_active.into_iter()))
            .filter_map(|(code, (business_line, is_active))| {
                if !is_active {
                    return None;
                }
                let bl = match business_line.as_str() {
                    "electronics" => 0,
                    "grocery" => 1,
                    "pharmacy" => 2,
                    "restaurant" => 3,
                    _ => return None,
                };
                if code == "" {
                    return None;
                }
                for c in code.chars() {
                    if c != '_' && !c.is_ascii_alphanumeric() {
                        return None;
                    }
                }
                return Some((bl, code));
            })
            .collect::<Vec<_>>();
        res.sort();
        res.into_iter().map(|(_, code)| code).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn coupon_code_validator() {
        assert_eq!(
            Solution::validate_coupons(
                vec_str!["SAVE20", "", "PHARMA5", "SAVE@20"],
                vec_str!["restaurant", "grocery", "pharmacy", "restaurant"],
                vec![true, true, true, true]
            ),
            ["PHARMA5", "SAVE20"]
        );
        assert_eq!(
            Solution::validate_coupons(
                vec_str!["GROCERY15", "ELECTRONICS_50", "DISCOUNT10"],
                vec_str!["grocery", "electronics", "invalid"],
                vec![false, true, true]
            ),
            ["ELECTRONICS_50"]
        );
    }
}
