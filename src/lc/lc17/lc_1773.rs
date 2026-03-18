// https://leetcode.com/problems/count-items-matching-a-rule/
// 1773. Count Items Matching a Rule
pub struct Solution;
impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let idx = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => unreachable!(),
        };
        items.iter().filter(|item| item[idx] == rule_value).count() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_matches() {
        assert_eq!(
            Solution::count_matches(
                vec_vec_str![
                    ["phone", "blue", "pixel"],
                    ["computer", "silver", "lenovo"],
                    ["phone", "gold", "iphone"]
                ],
                "color".to_string(),
                "silver".to_string()
            ),
            1
        );
        assert_eq!(
            Solution::count_matches(
                vec_vec_str![
                    ["phone", "blue", "pixel"],
                    ["computer", "silver", "lenovo"],
                    ["phone", "gold", "iphone"]
                ],
                "type".to_string(),
                "phone".to_string()
            ),
            2
        );
    }
}
