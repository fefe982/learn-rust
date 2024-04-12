// https://leetcode.com/problems/find-champion-i/
// 2923. Find Champion I
pub struct Solution;
impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let mut v = vec![0; grid.len()];
        for i in 1..grid.len() {
            for j in 0..i {
                if grid[i][j] == 1 {
                    v[j] += 1;
                } else {
                    v[i] += 1;
                }
            }
        }
        for (i, c) in v.into_iter().enumerate() {
            if c == 0 {
                return i as i32;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_champion() {
        assert_eq!(Solution::find_champion(vec_vec![[0, 1], [0, 0]]), 0);
        assert_eq!(Solution::find_champion(vec_vec![[0, 0, 1], [1, 0, 1], [0, 0, 0]]), 1);
    }
}
