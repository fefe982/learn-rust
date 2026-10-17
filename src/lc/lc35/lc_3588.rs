// https://leetcode.com/problems/find-maximum-area-of-a-triangle/
// 3588. Find Maximum Area of a Triangle
pub struct Solution;
impl Solution {
    pub fn max_area(coords: Vec<Vec<i32>>) -> i64 {
        let mut x_max = i32::MIN;
        let mut x_min = i32::MAX;
        let mut y_max = i32::MIN;
        let mut y_min = i32::MAX;
        let mut xm = std::collections::HashMap::new();
        let mut ym = std::collections::HashMap::new();
        for c in coords {
            let x = c[0];
            let y = c[1];
            x_max = x_max.max(x);
            x_min = x_min.min(x);
            y_max = y_max.max(y);
            y_min = y_min.min(y);
            let py = xm.entry(x).or_insert((i32::MAX, i32::MIN));
            py.0 = py.0.min(y);
            py.1 = py.1.max(y);
            let px = ym.entry(y).or_insert((i32::MAX, i32::MIN));
            px.0 = px.0.min(x);
            px.1 = px.1.max(x);
        }
        let mut res = 0;
        for (&x, &py) in &xm {
            res = res.max((py.1 - py.0) as i64 * (x - x_min).max(x_max - x) as i64);
        }
        for (&y, &px) in &ym {
            res = res.max((px.1 - px.0) as i64 * (y - y_min).max(y_max - y) as i64);
        }
        if res == 0 {
            -1
        } else {
            res
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_area() {
        assert_eq!(Solution::max_area(vec_vec![[1, 1], [1, 2], [3, 2], [3, 3]]), 2);
        assert_eq!(Solution::max_area(vec_vec![[1, 1], [2, 2], [3, 3]]), -1);
    }
}
