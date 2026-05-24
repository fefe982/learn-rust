// https://leetcode.com/problems/cells-in-a-range-on-an-excel-sheet/
// 2194. Cells in a Range on an Excel Sheet
pub struct Solution;
impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut iter = s.split(':');
        let left = iter.next().unwrap();
        let right = iter.next().unwrap();
        for c in left.chars().next().unwrap()..=right.chars().next().unwrap() {
            for d in left[1..].parse::<u8>().unwrap()..=right[1..].parse::<u8>().unwrap() {
                ans.push(format!("{}{}", c, d));
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_cells_in_range() {
        assert_eq!(
            Solution::cells_in_range("K1:L2".to_string()),
            vec_str!["K1", "K2", "L1", "L2"]
        );
        assert_eq!(
            Solution::cells_in_range("A1:F1".to_string()),
            vec_str!["A1", "B1", "C1", "D1", "E1", "F1"]
        );
    }
}
