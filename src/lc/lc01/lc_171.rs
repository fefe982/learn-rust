// https://leetcode.com/problems/excel-sheet-column-number/
// 171. Excel Sheet Column Number
pub struct Solution;
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut res = 0;
        for c in column_title.chars() {
            res = res * 26 + (c as u8 - b'A' + 1) as i32;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_title_to_number() {
        assert_eq!(Solution::title_to_number("A".to_string()), 1);
        assert_eq!(Solution::title_to_number("AB".to_string()), 28);
        assert_eq!(Solution::title_to_number("ZY".to_string()), 701);
    }
}
