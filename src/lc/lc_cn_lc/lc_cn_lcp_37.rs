// https://leetcode.cn/problems/zui-xiao-ju-xing-mian-ji/
// LCP 37. 最小矩形面积
pub struct Solution;
impl Solution {
    pub fn min_rec_size(lines: Vec<Vec<i32>>) -> f64 {
        if lines.len() <= 2 {
            return 0.0;
        }
        let mut lines = lines;
        lines.sort();
        if lines[0][0] == lines[lines.len() - 1][0] {
            return 0.0;
        }
        let mut minx = f64::MAX;
        let mut maxx = f64::MIN;
        let mut miny = f64::MAX;
        let mut maxy = f64::MIN;
        let mut i = 0;
        let mut j = 1;
        while lines[j][0] == lines[i][0] {
            j += 1;
        }
        let mut k = j + 1;
        while k < lines.len() && lines[k][0] == lines[j][0] {
            k += 1;
        }
        let xx =
            |a: usize, b: usize| -> f64 { (lines[a][1] - lines[b][1]) as f64 / (lines[a][0] - lines[b][0]) as f64 };
        let yy = |a: usize, b: usize| -> f64 {
            (lines[a][0] * lines[b][1] - lines[b][0] * lines[a][1]) as f64 / (lines[a][0] - lines[b][0]) as f64
        };
        loop {
            let x1 = xx(i, k - 1);
            let x2 = xx(j - 1, j);
            let y1 = yy(i, k - 1);
            let y2 = yy(j - 1, j);
            minx = minx.min(x1).min(x2);
            maxx = maxx.max(x1).max(x2);
            miny = miny.min(y1).min(y2);
            maxy = maxy.max(y1).max(y2);
            if k >= lines.len() {
                break;
            }
            i = j;
            j = k;
            k = j + 1;
            while k < lines.len() && lines[k][0] == lines[j][0] {
                k += 1;
            }
        }
        (maxy - miny) * (maxx - minx)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn min_rec_size() {
        assert_approx_eq!(Solution::min_rec_size(vec_vec![[2, 3], [3, 0], [4, 1]]), 48.0, 1e-4);
        assert_approx_eq!(Solution::min_rec_size(vec_vec![[1, 1], [2, 3]]), 0.0, 1e-4);
    }
}
