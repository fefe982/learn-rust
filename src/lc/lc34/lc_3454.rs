// https://leetcode.com/problems/separate-squares-ii/
// 3454. Separate Squares II
pub struct Solution;
impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let n = squares.len();
        let mut xset = std::collections::BTreeSet::new();
        let mut yevent = Vec::with_capacity(n * 2);
        for sq in squares {
            let x = sq[0];
            let y = sq[1];
            let l = sq[2];
            let xl = x + l;
            xset.insert(x);
            xset.insert(xl);
            yevent.push((y, 1, x, xl));
            yevent.push((y + l, -1, x, xl));
        }
        let xv = xset.into_iter().collect::<Vec<_>>();
        let nx = xv.len();
        let tree_bound = 1 << (usize::BITS - (nx - 1).leading_zeros());
        let mut occupy = vec![0; tree_bound * 2];
        let mut cover_cnt = vec![0; tree_bound * 2];
        fn update(
            occupy: &mut Vec<i32>,
            cover_cnt: &mut Vec<i32>,
            xv: &Vec<i32>,
            idx: usize,
            l: usize,
            r: usize,
            xl: i32,
            xr: i32,
            delta: i32,
        ) {
            if xv[l] >= xr || xv[r] <= xl {
                return;
            }
            if xl <= xv[l] && xv[r] <= xr {
                cover_cnt[idx] += delta;
            } else {
                let mid = (l + r) >> 1;
                update(occupy, cover_cnt, xv, idx * 2 + 1, l, mid, xl, xr, delta);
                update(occupy, cover_cnt, xv, idx * 2 + 2, mid, r, xl, xr, delta);
            }
            if cover_cnt[idx] > 0 {
                occupy[idx] = xv[r] - xv[l];
            } else if l + 1 == r {
                occupy[idx] = 0;
            } else {
                occupy[idx] = occupy[idx * 2 + 1] + occupy[idx * 2 + 2];
            }
        }
        yevent.sort();
        let mut sumv = Vec::with_capacity(yevent.len());
        let mut widthv = Vec::with_capacity(yevent.len());
        let mut yprev = 0;
        let mut area = 0;
        for &(y, delta, xl, xr) in &yevent {
            area += (y - yprev) as i64 * occupy[0] as i64;
            update(&mut occupy, &mut cover_cnt, &xv, 0, 0, nx - 1, xl, xr, delta);
            yprev = y;
            sumv.push(area);
            widthv.push(occupy[0]);
        }
        let idx = sumv.partition_point(|&a| a < (area + 1) / 2) - 1;
        let h = yevent[idx].0 as f64;
        let w = widthv[idx] as f64;
        h + (area - sumv[idx] * 2) as f64 / w / 2.0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn separate_squares() {
        assert_approx_eq!(
            Solution::separate_squares(vec_vec![[15, 21, 2], [19, 21, 3]]),
            22.3,
            1e-5
        );
        assert_approx_eq!(Solution::separate_squares(vec_vec![[0, 0, 1], [2, 2, 1]]), 1.0, 1e-5);
        assert_approx_eq!(Solution::separate_squares(vec_vec![[0, 0, 2], [1, 1, 1]]), 1.0, 1e-5);
    }
}
