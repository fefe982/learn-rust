// https://leetcode.com/problems/intersection-lcci/
// 面试题 16.03. Intersection LCCI
pub struct Solution;
impl Solution {
    pub fn intersection(start1: Vec<i32>, end1: Vec<i32>, start2: Vec<i32>, end2: Vec<i32>) -> Vec<f64> {
        let dy1 = (end1[1] - start1[1]) as i64;
        let dx1 = (end1[0] - start1[0]) as i64;
        let dy2 = (end2[1] - start2[1]) as i64;
        let dx2 = (end2[0] - start2[0]) as i64;
        fn cross(s1: i64, e1: i64, s2: i64, e2: i64) -> i32 {
            let mut imin = -1;
            let mut min = i64::MAX;
            if (s1 - s2) * (s1 - e2) <= 0 {
                if s1 < min {
                    min = s1;
                    imin = 0;
                }
            }
            if (e1 - s2) * (e1 - e2) <= 0 {
                if e1 < min {
                    min = e1;
                    imin = 1;
                }
            }
            if (s2 - s1) * (s2 - e1) <= 0 {
                if s2 < min {
                    min = s2;
                    imin = 2;
                }
            }
            if (e2 - s1) * (e2 - e1) <= 0 {
                if e2 < min {
                    imin = 3;
                }
            }
            imin
        }
        let pick = |imin: i32| -> Vec<f64> {
            match imin {
                0 => vec![start1[0] as f64, start1[1] as f64],
                1 => vec![end1[0] as f64, end1[1] as f64],
                2 => vec![start2[0] as f64, start2[1] as f64],
                3 => vec![end2[0] as f64, end2[1] as f64],
                _ => vec![],
            }
        };
        if dy1 * dx2 == dy2 * dx1 {
            let dy = (start2[1] - start1[1]) as i64;
            let dx = (start2[0] - start1[0]) as i64;
            if dy * dx2 == dx * dy2 {
                if start1[0] == end1[0] {
                    let imin = cross(start1[1] as i64, end1[1] as i64, start2[1] as i64, end2[1] as i64);
                    pick(imin)
                } else {
                    let imin = cross(start1[0] as i64, end1[0] as i64, start2[0] as i64, end2[0] as i64);
                    pick(imin)
                }
            } else {
                vec![]
            }
        } else {
            let denorm = dx2 * dy1 - dy2 * dx1;
            let c1 = start1[0] as i64 * end1[1] as i64 - end1[0] as i64 * start1[1] as i64;
            let c2 = start2[0] as i64 * end2[1] as i64 - end2[0] as i64 * start2[1] as i64;
            let yy = (c1 * dy2 - c2 * dy1) as f64 / denorm as f64;
            let xx = (c1 * dx2 - c2 * dx1) as f64 / denorm as f64;
            fn is_in(x: f64, s: i32, e: i32) -> bool {
                let s = s as f64;
                let e = e as f64;
                (s - x).abs() < 1e-6 || (e - x).abs() < 1e-6 || (x - s) * (x - e) < 0.0
            }
            if is_in(xx, start1[0], end1[0])
                && is_in(xx, start2[0], end2[0])
                && is_in(yy, start1[1], end1[1])
                && is_in(yy, start2[1], end2[1])
            {
                vec![xx, yy]
            } else {
                vec![]
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    fn assert_approx_eq_vec(a: Vec<f64>, b: Vec<f64>) {
        println!("{a:?} {b:?}");
        assert_eq!(a.len(), b.len());
        for i in 0..a.len() {
            assert_approx_eq!(a[i], b[i], 1e-6);
        }
    }
    #[test]
    fn intersection() {
        assert_approx_eq_vec(
            Solution::intersection(vec![0, 0], vec![1, 0], vec![1, 1], vec![0, -1]),
            vec![0.5, 0.0],
        );
        assert_approx_eq_vec(
            Solution::intersection(vec![0, 0], vec![3, 3], vec![1, 1], vec![2, 2]),
            vec![1.0, 1.0],
        );
        assert_approx_eq_vec(
            Solution::intersection(vec![0, 0], vec![1, 1], vec![1, 0], vec![2, 1]),
            vec![],
        );
    }
}
