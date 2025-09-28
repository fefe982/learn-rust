// https://leetcode.com/problems/matrix-cells-in-distance-order/
// 1030. Matrix Cells in Distance Order
pub struct Solution;
impl Solution {
    pub fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::with_capacity(rows as usize * cols as usize);
        for r in 0..rows {
            for c in 0..cols {
                res.push(vec![r, c]);
            }
        }
        res.sort_by_key(|x| (x[0] - r_center).abs() + (x[1] - c_center).abs());
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(rows: i32, cols: i32, r_center: i32, c_center: i32) {
        let res = Solution::all_cells_dist_order(rows, cols, r_center, c_center);
        let set: std::collections::HashSet<(i32, i32)> =
            std::collections::HashSet::from_iter(res.iter().map(|x| (x[0], x[1])));
        assert_eq!(set.len(), res.len());
        assert_eq!(set.len(), rows as usize * cols as usize);
        let mut d = 0;
        for v in res {
            let r = v[0];
            let c = v[1];
            assert!(r >= 0 && r < rows);
            assert!(c >= 0 && c < cols);
            let dd = (r - r_center).abs() + (c - c_center).abs();
            assert!(dd >= d);
            d = dd;
        }
    }
    #[test]
    fn all_cells_dist_order() {
        check(1, 2, 0, 0);
        check(2, 2, 0, 1);
        check(2, 3, 1, 2);
    }
}
