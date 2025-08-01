// https://leetcode.com/problems/delete-greatest-value-in-each-row/
// 2500. Delete Greatest Value in Each Row
pub struct Solution;
impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        grid.iter_mut().for_each(|v| v.sort_unstable());
        let mut sum = 0;
        for j in 0..grid[0].len() {
            let mut max = i32::MIN;
            for i in 0..grid.len() {
                max = max.max(grid[i][j]);
            }
            sum += max;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn delete_greatest_value() {
        assert_eq!(Solution::delete_greatest_value(vec_vec![[1, 2, 4], [3, 3, 1]]), 8);
        assert_eq!(Solution::delete_greatest_value(vec_vec![[10]]), 10);
    }
}
