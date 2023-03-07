// https://leetcode.com/problems/count-total-number-of-colored-cells/
// 2579. Count Total Number of Colored Cells
pub struct Solution;
impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        2 * n as i64 * n as i64 - 2 * n as i64 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn colored_cells() {
        assert_eq!(Solution::colored_cells(1), 1);
        assert_eq!(Solution::colored_cells(2), 5);
    }
}
