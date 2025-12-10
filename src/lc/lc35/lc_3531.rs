// https://leetcode.com/problems/count-covered-buildings/
// 3531. Count Covered Buildings
pub struct Solution;
impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut row_min = vec![i32::MAX; n + 1];
        let mut row_max = vec![i32::MIN; n + 1];
        let mut col_min = vec![i32::MAX; n + 1];
        let mut col_max = vec![i32::MIN; n + 1];
        for b in &buildings {
            row_min[b[0] as usize] = row_min[b[0] as usize].min(b[1]);
            row_max[b[0] as usize] = row_max[b[0] as usize].max(b[1]);
            col_min[b[1] as usize] = col_min[b[1] as usize].min(b[0]);
            col_max[b[1] as usize] = col_max[b[1] as usize].max(b[0]);
        }
        let mut ans = 0;
        for b in buildings {
            if b[1] > row_min[b[0] as usize]
                && b[1] < row_max[b[0] as usize]
                && b[0] > col_min[b[1] as usize]
                && b[0] < col_max[b[1] as usize]
            {
                ans += 1;
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
    fn count_covered_buildings() {
        assert_eq!(
            Solution::count_covered_buildings(3, vec_vec![[1, 2], [2, 2], [3, 2], [2, 1], [2, 3]]),
            1
        );
        assert_eq!(
            Solution::count_covered_buildings(3, vec_vec![[1, 1], [1, 2], [2, 1], [2, 2]]),
            0
        );
        assert_eq!(
            Solution::count_covered_buildings(5, vec_vec![[1, 3], [3, 2], [3, 3], [3, 5], [5, 3]]),
            1
        );
    }
}
