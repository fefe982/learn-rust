// https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones-ii/
// 3197. Find the Minimum Area to Cover All Ones II
pub struct Solution;
impl Solution {
    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut cols = vec![vec![]; grid[0].len()];
        let mut rows = vec![vec![]; grid.len()];
        let mut col_lim_l = vec![(i32::MAX, 0); grid[0].len()];
        let mut col_lim_r = vec![(i32::MAX, 0); grid[0].len()];
        let mut row_lim_t = vec![(i32::MAX, 0); grid.len()];
        let mut row_lim_b = vec![(i32::MAX, 0); grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    cols[j].push(i as i32);
                    rows[i].push(j as i32);
                }
            }
        }
        let mut top_bound = usize::MAX;
        for i in 0..grid.len() {
            if i > 0 {
                row_lim_t[i] = row_lim_t[i - 1];
            }
            if rows[i].len() > 0 {
                top_bound = top_bound.min(i);
                row_lim_t[i] = (
                    row_lim_t[i].0.min(rows[i][0]),
                    row_lim_t[i].1.max(rows[i][rows[i].len() - 1]),
                );
            }
        }
        let mut bot_bound = 0;
        for i in (0..grid.len()).rev() {
            if i < grid.len() - 1 {
                row_lim_b[i] = row_lim_b[i + 1];
            }
            if rows[i].len() > 0 {
                bot_bound = bot_bound.max(i);
                row_lim_b[i] = (
                    row_lim_b[i].0.min(rows[i][0]),
                    row_lim_b[i].1.max(rows[i][rows[i].len() - 1]),
                );
            }
        }
        let mut left_bound = usize::MAX;
        for i in 0..grid[0].len() {
            if i > 0 {
                col_lim_l[i] = col_lim_l[i - 1];
            }
            if cols[i].len() > 0 {
                left_bound = left_bound.min(i);
                col_lim_l[i] = (
                    col_lim_l[i].0.min(cols[i][0]),
                    col_lim_l[i].1.max(cols[i][cols[i].len() - 1]),
                );
            }
        }
        let mut right_bound = 0;
        for i in (0..grid[0].len()).rev() {
            if i < grid[0].len() - 1 {
                col_lim_r[i] = col_lim_r[i + 1];
            }
            if cols[i].len() > 0 {
                right_bound = right_bound.max(i);
                col_lim_r[i] = (
                    col_lim_r[i].0.min(cols[i][0]),
                    col_lim_r[i].1.max(cols[i][cols[i].len() - 1]),
                )
            }
        }
        let mut ans = i32::MAX;
        ans = ans.min(Self::split3(left_bound, right_bound, &cols, &col_lim_l, &col_lim_r));
        ans = ans.min(Self::split3(top_bound, bot_bound, &rows, &row_lim_t, &row_lim_b));
        ans = ans.min(Self::split12(
            left_bound,
            right_bound,
            &cols,
            &col_lim_l,
            top_bound,
            bot_bound,
            &rows,
            &row_lim_t,
            &row_lim_b,
        ));
        ans = ans.min(Self::split12(
            right_bound,
            left_bound,
            &cols,
            &col_lim_r,
            top_bound,
            bot_bound,
            &rows,
            &row_lim_t,
            &row_lim_b,
        ));
        ans = ans.min(Self::split12(
            top_bound,
            bot_bound,
            &rows,
            &row_lim_t,
            left_bound,
            right_bound,
            &cols,
            &col_lim_l,
            &col_lim_r,
        ));
        ans = ans.min(Self::split12(
            bot_bound,
            top_bound,
            &rows,
            &row_lim_b,
            left_bound,
            right_bound,
            &cols,
            &col_lim_l,
            &col_lim_r,
        ));
        ans as i32
    }

    fn split3(
        bound0: usize,
        bound1: usize,
        idxs: &Vec<Vec<i32>>,
        lim0: &Vec<(i32, i32)>,
        lim1: &Vec<(i32, i32)>,
    ) -> i32 {
        let mut ans = i32::MAX;
        for i in bound0..bound1 {
            if idxs[i].len() == 0 {
                continue;
            }
            let mut mid = i + 1;
            while mid < bound1 && idxs[mid].len() == 0 {
                mid += 1;
            }
            if mid >= bound1 {
                break;
            }
            let mut mid_upper = i32::MAX;
            let mut mid_lower = i32::MIN;
            for j in mid..bound1 {
                if idxs[j].len() == 0 {
                    continue;
                }
                mid_upper = mid_upper.min(idxs[j][0]);
                mid_lower = mid_lower.max(idxs[j][idxs[j].len() - 1]);
                let mut right = j + 1;
                while right < bound1 && idxs[right].len() == 0 {
                    right += 1;
                }
                if right > bound1 {
                    break;
                }
                ans = ans.min(
                    (lim0[i].1 - lim0[i].0 + 1) * (i - bound0 + 1) as i32
                        + (mid_lower - mid_upper + 1) * (j - mid + 1) as i32
                        + (lim1[right].1 - lim1[right].0 + 1) * (bound1 - right + 1) as i32,
                );
            }
        }
        ans
    }
    fn split12(
        bound0: usize,
        bound1: usize,
        idxs: &Vec<Vec<i32>>,
        lim0: &Vec<(i32, i32)>,
        bound_vert0: usize,
        bound_vert1: usize,
        idx_vert: &Vec<Vec<i32>>,
        lim_vert0: &Vec<(i32, i32)>,
        lim_vert1: &Vec<(i32, i32)>,
    ) -> i32 {
        let mut ans = i32::MAX;
        let (len, dir, mut cidx) = if bound1 > bound0 {
            (bound1 - bound0 + 1, 1, vec![0; idx_vert.len()])
        } else {
            (
                bound0 - bound1 + 1,
                -1,
                idx_vert.iter().map(|v| v.len() as i32 - 1).collect(),
            )
        };
        for ii in 0..len {
            let i = (bound0 as i32 + ii as i32 * dir) as usize;
            if idxs[i].len() == 0 {
                continue;
            }
            let mut heap = std::collections::BinaryHeap::new();
            let mut lower = usize::MAX;
            let mut upper = 0;
            for j in bound_vert0..=bound_vert1 {
                if cidx[j] >= 0 && cidx[j] < idx_vert[j].len() as i32 {
                    if idx_vert[j][cidx[j] as usize] == i as i32 {
                        cidx[j] += dir;
                    }
                    if cidx[j] >= 0 && cidx[j] < idx_vert[j].len() as i32 {
                        lower = lower.min(j);
                        upper = upper.max(j);
                        heap.push((-idx_vert[j][cidx[j] as usize] * dir, j));
                    }
                }
            }
            let mut min_upper = i32::MAX;
            for j in lower..upper {
                if cidx[j] < 0 || cidx[j] >= idx_vert[j].len() as i32 {
                    continue;
                }
                min_upper = min_upper.min(idx_vert[j][cidx[j] as usize] * dir);
                let mut k = j + 1;
                while k <= upper && (cidx[k] < 0 || cidx[k] >= idx_vert[k].len() as i32) {
                    k += 1;
                }
                if k > upper {
                    break;
                }
                while let Some(&(top, hj)) = heap.peek() {
                    if hj < k {
                        heap.pop();
                    } else {
                        ans = ans.min(
                            (lim0[i].1 - lim0[i].0 + 1) * (ii + 1) as i32
                                + (if dir == 1 { lim_vert0[j].1 } else { -lim_vert0[j].0 } - min_upper + 1)
                                    * (j - lower + 1) as i32
                                + (if dir == 1 { lim_vert1[k].1 } else { -lim_vert1[k].0 } + top + 1)
                                    * (upper - k + 1) as i32,
                        );
                        break;
                    }
                }
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
    fn minimum_sum() {
        assert_eq!(Solution::minimum_sum(vec_vec![[1, 1], [1, 0]]), 3);
        assert_eq!(Solution::minimum_sum(vec_vec![[0, 1], [1, 1]]), 3);
        assert_eq!(Solution::minimum_sum(vec_vec![[1, 0, 1], [1, 1, 1]]), 5);
        assert_eq!(Solution::minimum_sum(vec_vec![[1, 0, 1, 0], [0, 1, 0, 1]]), 5);
    }
}
