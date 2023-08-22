// https://leetcode.com/problems/excel-sheet-column-title/
// 168. Excel Sheet Column Title
pub struct Solution;
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let voc = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut i = column_number as usize;
        let mut v = vec![];
        while i > 0 {
            let d = (i - 1) % 26;
            i = (i - 1) / 26;
            v.insert(0, voc[d]);
        }
        String::from_utf8(v).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn convert_to_title() {
        assert_eq!(Solution::convert_to_title(1), "A");
        assert_eq!(Solution::convert_to_title(28), "AB");
        assert_eq!(Solution::convert_to_title(701), "ZY");
    }
}
