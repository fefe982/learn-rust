// https://leetcode.com/problems/pascals-triangle-ii/
// 119. Pascal's Triangle II
pub struct Solution;
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut v = vec![0; row_index as usize + 1];
        v[0] = 1;
        for i in 1..=row_index as usize {
            for j in (1..=i).rev() {
                v[j] = v[j - 1] + v[j];
            }
        }
        v
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_row() {
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(1), vec![1, 1]);
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }
}
