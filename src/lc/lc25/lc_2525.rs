// https://leetcode.com/problems/categorize-box-according-to-criteria/
// 2525. Categorize Box According to Criteria
pub struct Solution;
impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        match (
            length >= 10000
                || width >= 10000
                || height >= 10000
                || length as i64 * width as i64 * height as i64 >= 10_0000_0000,
            mass >= 100,
        ) {
            (true, true) => "Both",
            (true, false) => "Bulky",
            (false, true) => "Heavy",
            (false, false) => "Neither",
        }
        .to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorize_box() {
        assert_eq!(Solution::categorize_box(1000, 35, 700, 300), "Heavy");
        assert_eq!(Solution::categorize_box(200, 50, 800, 50), "Neither");
    }
}
