// https://leetcode.com/problems/check-if-the-rectangle-corner-is-reachable/
// 3235. Check if the Rectangle Corner is Reachable
pub struct Solution;
impl Solution {
    pub fn can_reach_corner(x_corner: i32, y_corner: i32, circles: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![false; circles.len()];
        let in_circle = |x: i32, y: i32, r: i32, xc: i32, yc: i32| {
            let dx = (xc - x) as i64;
            let dy = (yc - y) as i64;
            let r = r as i64;
            dx * dx + dy * dy <= r * r
        };
        let touch = |x1: i32, y1: i32, r1: i32, x2: i32, y2: i32, r2: i32, xc: i32, yc: i32| {
            let dx = (x1 - x2) as i64;
            let dy = (y1 - y2) as i64;
            let r1 = r1 as i64;
            let r2 = r2 as i64;
            let sr = r1 + r2;
            let xc = xc as i64;
            let yc = yc as i64;
            let x1 = x1 as i64;
            let x2 = x2 as i64;
            let y1 = y1 as i64;
            let y2 = y2 as i64;
            dx * dx + dy * dy <= sr * sr && x1 * r2 + x2 * r1 < sr * xc && y1 * r2 + y2 * r1 < sr * yc
        };
        for i in 0..circles.len() {
            let xi = circles[i][0];
            let yi = circles[i][1];
            let ri = circles[i][2];
            if in_circle(xi, yi, ri, 0, 0) || in_circle(xi, yi, ri, x_corner, y_corner) {
                return false;
            }
            if visited[i] {
                continue;
            }
            if (xi <= x_corner && (yi - y_corner).abs() <= ri)
                || (yi <= y_corner && xi <= ri)
                || (yi > y_corner && in_circle(xi, yi, ri, 0, y_corner))
            {
                let mut q = vec![i];
                while let Some(c) = q.pop() {
                    if visited[c] {
                        continue;
                    }
                    visited[c] = true;
                    let xc = circles[c][0];
                    let yc = circles[c][1];
                    let rc = circles[c][2];
                    if (yc <= y_corner && (xc - x_corner).abs() <= rc)
                        || (xc <= x_corner && yc <= rc)
                        || (xc > x_corner && in_circle(xc, yc, rc, x_corner, 0))
                    {
                        return false;
                    }
                    for j in 0..circles.len() {
                        if visited[j] {
                            continue;
                        }
                        if touch(
                            xc,
                            yc,
                            rc,
                            circles[j][0],
                            circles[j][1],
                            circles[j][2],
                            x_corner,
                            y_corner,
                        ) {
                            q.push(j);
                        }
                    }
                }
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_can_reach_corner() {
        assert_eq!(
            Solution::can_reach_corner(8, 8, vec_vec![[1, 4, 1], [3, 4, 1], [5, 4, 1], [7, 4, 1]]),
            false
        );
        assert_eq!(Solution::can_reach_corner(3, 4, vec_vec![[2, 1, 1]]), true);
        assert_eq!(Solution::can_reach_corner(3, 3, vec_vec![[1, 1, 2]]), false);
        assert_eq!(Solution::can_reach_corner(3, 3, vec_vec![[2, 1, 1], [1, 2, 1]]), false);
        assert_eq!(Solution::can_reach_corner(4, 4, vec_vec![[5, 5, 1]]), true);
    }
}
